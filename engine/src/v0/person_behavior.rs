use crate::v0::core::Person;
use crate::v0::geometry::{BoundingBox, Position};
use crate::v0::maps;
use crate::v0::maps::MapElement;
use anyhow::{anyhow, Result};
use pathfinding::prelude::astar;
use rand::{Rng, RngCore};
use std::f32::consts::PI;
use std::iter::Iterator;

pub(crate) trait PersonBehavior {
    fn update_positions(
        &mut self,
        people: &mut [Person],
        map: &mut Option<maps::Map>,
        rng: &mut dyn RngCore,
    );
}

///////////////////////////////////////////////////////////////////////////////
// Brownian motion behaviors
///////////////////////////////////////////////////////////////////////////////

struct BrownianMotionPersonState {
    direction_rad: f32,
}

pub(crate) struct BrownianMotionBehavior {
    world_size: (u16, u16),
    per_person_states: Vec<BrownianMotionPersonState>,
}

impl BrownianMotionBehavior {
    pub(crate) fn new(world_size: (u16, u16), num_people: usize, rng: &mut dyn RngCore) -> Self {
        let per_person_states = (0..num_people)
            .map(|_| BrownianMotionPersonState {
                direction_rad: rng.gen_range(0.0, 2.0 * PI),
            })
            .collect();

        BrownianMotionBehavior {
            world_size,
            per_person_states,
        }
    }
}

impl PersonBehavior for BrownianMotionBehavior {
    fn update_positions(
        &mut self,
        people: &mut [Person],
        _: &mut Option<maps::Map>,
        _: &mut dyn RngCore,
    ) {
        for (idx, person) in people.iter_mut().enumerate() {
            let state = &mut self.per_person_states[idx];

            person
                .position
                .advance(&mut state.direction_rad, self.world_size)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Shopper behaviors
///////////////////////////////////////////////////////////////////////////////

enum ShopperState {
    AtHome {
        direction_rad: f32,
    },
    GoingToStore {
        path_idx: usize,
        path: Vec<(u16, u16)>,
    },
    Shopping {
        direction_rad: f32,
        cart_supply_levels: f32,
    },
    ReturningHome {
        cart_supply_levels: f32,
        path_idx: usize,
        path: Vec<(u16, u16)>,
    },
    FollowHeadOfHousehold,
}

struct HouseholdState {
    head_of_household_idx: usize,
    supply_levels: f32,
    single_shopper: bool,
}

pub(crate) struct ShopperBehavior {
    world_size: (u16, u16),
    per_person_states: Vec<ShopperState>,
    per_household_states: Vec<HouseholdState>,
}

impl ShopperBehavior {
    pub(crate) fn new(
        world_size: (u16, u16),
        people: &Vec<Person>,
        map: &maps::Map,
        rng: &mut dyn RngCore,
    ) -> Self {
        let mut per_household_states = (0..map.households.len())
            .map(|_| HouseholdState {
                head_of_household_idx: 0, // To be filled in.
                // TODO: load this from some config.
                supply_levels: rng.gen_range(150.0, 450.0),
                single_shopper: rng.gen_bool(0.5),
            })
            .collect::<Vec<_>>();

        let per_person_states = people
            .iter()
            .map(|p| {
                if p.head_of_household {
                    let household_state = &mut per_household_states[p.household_idx];
                    household_state.head_of_household_idx = p.id;
                }
                ShopperState::AtHome {
                    direction_rad: rng.gen_range(0.0, 2.0 * PI),
                }
            })
            .collect();

        ShopperBehavior {
            world_size,
            per_person_states,
            per_household_states,
        }
    }

    fn find_bb_road_intersection(
        world_size: (u16, u16),
        bb: &BoundingBox,
        map: &maps::Map,
    ) -> Vec<((u16, u16), (u16, u16))> {
        // Returns a list of a pair of points, such that
        //   the first point of the pair is inside the bounding box
        //   the second point of the pair is on the road
        //   the first and second points are adjacent
        let mut intersections = vec![];

        // Iterate over the top boundary of the bounding box.
        if bb.top_left.0 > 0 {
            let row = bb.top_left.0 - 1;
            for col in bb.cols() {
                if map.get_element(row, col) == maps::MapElement::Road {
                    intersections
                        .push(((bb.top_left.0 as u16, col as u16), (row as u16, col as u16)));
                }
            }
        }

        // Iterate over the left boundary of the bounding box.
        if bb.top_left.1 > 0 {
            let col = bb.top_left.1 - 1;
            for row in bb.rows() {
                if map.get_element(row, col) == maps::MapElement::Road {
                    intersections
                        .push(((row as u16, bb.top_left.1 as u16), (row as u16, col as u16)));
                }
            }
        }

        // Iterate over the bottom boundary of the bounding box
        if (bb.bottom_right.0 as u16) < world_size.1 {
            let row = bb.bottom_right.0;
            for col in bb.cols() {
                if map.get_element(row, col) == maps::MapElement::Road {
                    intersections.push((
                        ((bb.bottom_right.0 - 1) as u16, col as u16),
                        (row as u16, col as u16),
                    ));
                }
            }
        }

        // Iterate over the right boundary of the bounding box.
        if (bb.bottom_right.1 as u16) < world_size.0 {
            let col = bb.bottom_right.1;
            for row in bb.rows() {
                if map.get_element(row, col) == maps::MapElement::Road {
                    intersections.push((
                        (row as u16, (bb.bottom_right.1 - 1) as u16),
                        (row as u16, col as u16),
                    ));
                }
            }
        }

        intersections
    }

    fn find_path(
        starting: &Position,
        from_bb: &BoundingBox,
        to_bb: &BoundingBox,
        world_size: (u16, u16),
        map: &maps::Map,
        rng: &mut dyn RngCore,
    ) -> Result<Vec<(u16, u16)>> {
        // List of (from, road) points.
        let starting_intersections = Self::find_bb_road_intersection(world_size, from_bb, map);
        if starting_intersections.is_empty() {
            return Err(anyhow!("empty starting intersections"));
        }

        // List of (to, road) points
        let ending_intersections = Self::find_bb_road_intersection(world_size, to_bb, map);
        if ending_intersections.is_empty() {
            return Err(anyhow!("empty ending intersections"));
        }

        // Pick an arbitrary starting intersection and ending intersection.
        let starting_intersection_idx = rng.gen_range(0, starting_intersections.len());
        let starting_intersection = &starting_intersections[starting_intersection_idx];
        let starting_road_point = starting_intersection.1;

        let ending_intersection_idx = rng.gen_range(0, ending_intersections.len());
        let ending_intersection = &ending_intersections[ending_intersection_idx];
        let ending_road_point = ending_intersection.1;

        // Generate a path within the road from the starting_road_point to the ending_road_point

        let (road_path, _) = astar(
            &starting_road_point,
            |pos| {
                let mut successors = vec![];
                for (d_row, d_col) in vec![(-1, 0), (0, -1), (0, 1), (1, 0)] {
                    if d_row == -1 && pos.0 == 0 {
                        continue;
                    }
                    if d_row == 1 && pos.0 + 1 == world_size.1 {
                        continue;
                    }
                    if d_col == -1 && pos.1 == 0 {
                        continue;
                    }
                    if d_col == 1 && pos.1 + 1 == world_size.0 {
                        continue;
                    }

                    let candidate = ((pos.0 as i32 + d_row) as u16, (pos.1 as i32 + d_col) as u16);
                    if map.get_element(candidate.0 as usize, candidate.1 as usize)
                        == maps::MapElement::Road
                    {
                        successors.push((candidate, 1));
                    }
                }
                successors
            },
            |pos| {
                let d_row = (pos.0 as f32) - (ending_road_point.0 as f32);
                let d_col = (pos.1 as f32) - (ending_road_point.1 as f32);

                (d_row * d_row + d_col * d_col).sqrt() as u16
            },
            |pos| *pos == ending_road_point,
        )
        .ok_or(anyhow!("failed to find road path"))?;

        // Add intermediate nodes from the starting position to the starting intersection.
        // Search again.

        let mut starting_path = vec![Position {
            x: starting.x,
            y: starting.y,
        }];
        let starting_path_goal = Position {
            x: (starting_intersection.0).1 as f32,
            y: (starting_intersection.0).0 as f32,
        };

        let mut dx = starting_path_goal.x - starting.x;
        let mut dy = starting_path_goal.y - starting.y;
        let norm = (dx * dx + dy * dy).sqrt();

        dx /= norm;
        dy /= norm;

        loop {
            let head = &starting_path[starting_path.len() - 1];
            if head.distance(&starting_path_goal) < 1.0 {
                break;
            }
            let pos = Position {
                x: head.x + dx,
                y: head.y + dy,
            };
            starting_path.push(pos);
        }

        let mut entire_path = starting_path
            .into_iter()
            .map(|pos| (pos.y as u16, pos.x as u16))
            .collect::<Vec<_>>();

        entire_path.extend(road_path.into_iter());
        entire_path.push(ending_intersection.0);
        Ok(entire_path)
    }
}

impl PersonBehavior for ShopperBehavior {
    fn update_positions(
        &mut self,
        people: &mut [Person],
        maybe_map: &mut Option<maps::Map>,
        rng: &mut dyn RngCore,
    ) {
        // Step 0: Update the household supply levels.
        let map = maybe_map
            .as_mut()
            .expect("shopper behavior must have a map");
        for household_state in self.per_household_states.iter_mut() {
            household_state.supply_levels -= 1.0;
        }

        // Step 1: For each shopper, update their position:
        //   Case AtHome:
        //     Check the supply levels of the household.
        //     If they are acceptable, then brownian motion within the household.
        //     Otherwise, calculate a path towards the store and update state to GoingToStore.
        //   Case GoingToStore:
        //     If they are at the store, then advance the state to GoingToStore.
        //     Otherwise, continue on path towards the store.
        //   Case Shopping:
        //     Check how many supplies they have.
        //     If not enough, then brownian motion within the store and increase supply level.
        //     Otherwise, calculate a path towards home and update state to ReturningHome
        //   Case ReturningHome:
        //     If they are at home, then advance the state to Athome.
        //     Otherwise, continue on path towards home.
        for idx in 0..people.len() {
            let (left_people, right_people) = people.split_at_mut(idx);
            let person = &mut right_people[0];

            let (left_people_states, right_people_states) =
                self.per_person_states.split_at_mut(idx);
            // let state = &mut self.per_person_states[idx];
            let state = &mut right_people_states[0];
            match state {
                ShopperState::AtHome { direction_rad } => {
                    let household = &map.households[person.household_idx];
                    let household_state = &mut self.per_household_states[person.household_idx];
                    if household_state.supply_levels > 0.0 {
                        // Household supply levels are acceptable, brownian motion within household
                        person.position.advance2(direction_rad, &household.bounds);
                        continue;
                    }

                    if person.head_of_household {
                        let path = Self::find_path(
                            &person.position,
                            &household.bounds,
                            &map.stores[0].bounds,
                            self.world_size,
                            map,
                            rng,
                        )
                        .expect("failed to find path");
                        *state = ShopperState::GoingToStore { path_idx: 0, path };
                    } else {
                        if household_state.single_shopper {
                            // This household only has a single shopper (head of household).
                            // We are not the head, so do not shop.
                            person.position.advance2(direction_rad, &household.bounds);
                            continue;
                        }

                        *state = ShopperState::FollowHeadOfHousehold
                    }
                }
                ShopperState::GoingToStore { path_idx, path } => {
                    if *path_idx < path.len() {
                        person.position.x = path[*path_idx].1 as f32;
                        person.position.y = path[*path_idx].0 as f32;

                        if map.get_element(path[*path_idx].0 as usize, path[*path_idx].1 as usize)
                            == MapElement::Road
                        {
                            *path_idx += 3;
                        } else {
                            *path_idx += 1;
                        }
                    } else {
                        person.position.x = path[path.len() - 1].1 as f32;
                        person.position.y = path[path.len() - 1].0 as f32;
                        *state = ShopperState::Shopping {
                            direction_rad: rng.gen_range(0.0, 2.0 * PI),
                            cart_supply_levels: 0.0,
                        };
                    }
                }
                ShopperState::Shopping {
                    direction_rad,
                    cart_supply_levels,
                } => {
                    // TODO: stop hardcoding this limit
                    if *cart_supply_levels < 10.0 * 60.0 {
                        *cart_supply_levels += 1.0;
                        person
                            .position
                            .advance2(direction_rad, &map.stores[0].bounds);
                    } else {
                        let household = &map.households[person.household_idx];
                        let path = Self::find_path(
                            &person.position,
                            &map.stores[0].bounds,
                            &household.bounds,
                            self.world_size,
                            map,
                            rng,
                        )
                        .expect("failed to find path");
                        *state = ShopperState::ReturningHome {
                            cart_supply_levels: *cart_supply_levels,
                            path_idx: 0,
                            path,
                        };
                    }
                }
                ShopperState::ReturningHome {
                    cart_supply_levels,
                    path_idx,
                    path,
                } => {
                    if *path_idx < path.len() {
                        person.position.x = path[*path_idx].1 as f32;
                        person.position.y = path[*path_idx].0 as f32;

                        if map.get_element(path[*path_idx].0 as usize, path[*path_idx].1 as usize)
                            == MapElement::Road
                        {
                            *path_idx += 3;
                        } else {
                            *path_idx += 1;
                        }
                    } else {
                        person.position.x = path[path.len() - 1].1 as f32;
                        person.position.y = path[path.len() - 1].0 as f32;

                        let household_state = &mut self.per_household_states[person.household_idx];
                        household_state.supply_levels += 3.0 * *cart_supply_levels;

                        *state = ShopperState::AtHome {
                            direction_rad: rng.gen_range(0.0, 2.0 * PI),
                        };
                    }
                }
                ShopperState::FollowHeadOfHousehold => {
                    // If the head of household is at home and we are in the household, then
                    // transition to AtHome.
                    let household_state = &self.per_household_states[person.household_idx];
                    // This is sound because the head of household always has a lower index than
                    // anyone else in their household.
                    let head_of_household_state =
                        &left_people_states[household_state.head_of_household_idx];
                    if let ShopperState::AtHome { direction_rad: _ } = *head_of_household_state {
                        if map.get_element(person.position.y as usize, person.position.x as usize)
                            == maps::MapElement::Household
                        {
                            *state = ShopperState::AtHome {
                                direction_rad: rng.gen_range(0.0, 2.0 * PI),
                            };
                            continue;
                        }
                    }

                    // Otherwise, follow the head of household.
                    // This is sound because the head of household always has a lower index than
                    // anyone else in their household.
                    let head_pos = &left_people[household_state.head_of_household_idx].position;

                    if person.position.distance(head_pos) < 5.0 {
                        continue;
                    }

                    let dx = person.position.x - head_pos.x;
                    let dy = person.position.y - head_pos.y;
                    let norm = (dx * dx + dy * dy).sqrt();

                    person.position.x = head_pos.x + 5.0 * dx / norm;
                    person.position.y = head_pos.y + 5.0 * dy / norm;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v0::maps::simple_groceries;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_find_bb_road_intersection() -> Result<()> {
        let world_size = (600, 400);

        let sg_map = maps::Map::load_from_ascii_str(10, simple_groceries::MAP_ASCII_STR)?;
        let store = &sg_map.stores[0];

        let intersections =
            ShopperBehavior::find_bb_road_intersection(world_size, &store.bounds, &sg_map);
        assert_eq!(
            intersections,
            (290..310)
                .map(|col| ((100, col), (99, col),))
                .collect::<Vec<_>>()
        );

        let box_with_left_intersection = BoundingBox {
            top_left: (110, 60),
            bottom_right: (140, 90),
        };
        let intersections = ShopperBehavior::find_bb_road_intersection(
            world_size,
            &box_with_left_intersection,
            &sg_map,
        );
        assert_eq!(
            intersections,
            (120..130)
                .map(|row| ((row, 60), (row, 59)))
                .collect::<Vec<_>>()
        );

        // TODO: tests for box with top/right/bottom intersection.

        Ok(())
    }

    #[test]
    fn test_find_path() -> Result<()> {
        let mut rng = Box::new(ChaCha8Rng::seed_from_u64(10914));
        let world_size = (600, 400);

        let sg_map = maps::Map::load_from_ascii_str(10, simple_groceries::MAP_ASCII_STR)?;
        let store_bb = &sg_map.stores[0].bounds;

        let household_bb = BoundingBox {
            top_left: (110, 60),
            bottom_right: (140, 90),
        };

        let path = ShopperBehavior::find_path(
            &Position { x: 65.0, y: 125.0 },
            &household_bb,
            store_bb,
            world_size,
            &sg_map,
            &mut rng,
        )?;

        assert_eq!(path.len(), 397);
        assert_eq!(path[0], (125, 65));
        for window in path[1..].windows(2) {
            let pos1 = window[0];
            let pos2 = window[1];
            let d_row = pos2.0 as i32 - pos1.0 as i32;
            let d_col = pos2.1 as i32 - pos1.1 as i32;

            let el1 = sg_map.get_element(pos1.0 as usize, pos1.1 as usize);
            let el2 = sg_map.get_element(pos2.0 as usize, pos2.1 as usize);

            match (el1, el2) {
                (maps::MapElement::Household, maps::MapElement::Household) => {
                    match (d_row, d_col) {
                        (-1, -1) | (-1, 1) | (1, -1) | (1, 1) => (),
                        (-1, 0) | (0, -1) | (0, 1) | (1, 0) => (),
                        _ => panic!("invalid change in position: {} {}", d_row, d_col),
                    }
                }
                _ => match (d_row, d_col) {
                    (-1, 0) | (0, -1) | (0, 1) | (1, 0) => (),
                    _ => panic!("invalid change in position: {} {}", d_row, d_col),
                },
            }
        }

        for pos in path[6..path.len() - 1].iter() {
            assert_eq!(
                sg_map.get_element(pos.0 as usize, pos.1 as usize),
                maps::MapElement::Road
            );
        }
        let last_pos = path[path.len() - 1];
        assert_eq!(
            sg_map.get_element(last_pos.0 as usize, last_pos.1 as usize),
            maps::MapElement::Store
        );

        Ok(())
    }
}
