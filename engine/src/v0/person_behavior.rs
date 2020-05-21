use crate::v0::config::ShopperParams;
use crate::v0::core::Person;
use crate::v0::geometry::{BoundingBox, Position};
use crate::v0::maps::MapElement;
use crate::v0::utils::random_bool_vec;
use crate::v0::{maps, wasm_view};
use anyhow::Result;
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

    fn update_household_state(&self, _idx: usize, _state: &mut wasm_view::HouseholdState) {
        unimplemented!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Brownian motion behaviors
///////////////////////////////////////////////////////////////////////////////

struct BrownianMotionPersonState {
    direction_rad: f32,
}

pub(crate) struct BrownianMotionBehavior {
    world_bounding_box: BoundingBox,
    per_person_states: Vec<BrownianMotionPersonState>,
}

impl BrownianMotionBehavior {
    pub(crate) fn new(
        world_bounding_box: BoundingBox,
        num_people: usize,
        rng: &mut dyn RngCore,
    ) -> Self {
        let per_person_states = (0..num_people)
            .map(|_| BrownianMotionPersonState {
                direction_rad: rng.gen_range(0.0, 2.0 * PI),
            })
            .collect();

        BrownianMotionBehavior {
            world_bounding_box,
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
                .advance(&mut state.direction_rad, &self.world_bounding_box)
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
        shopping_duration_ticks: usize,
    },
    ReturningHome {
        path_idx: usize,
        path: Vec<(u16, u16)>,
    },
    FollowHeadOfHousehold,
}

struct HouseholdState {
    head_of_household_idx: usize,

    dual_shopper: bool,

    shopping_period_ticks: usize,
    supplies_bought_per_trip: f32,

    supply_levels: f32,
}

pub(crate) struct ShopperBehavior {
    per_person_states: Vec<ShopperState>,
    per_household_states: Vec<HouseholdState>,
}

impl ShopperBehavior {
    pub(crate) fn new(
        params: ShopperParams,
        people: &[Person],
        map: &maps::Map,
        rng: &mut dyn RngCore,
    ) -> Self {
        let dual_shopper_households = random_bool_vec(
            map.households.len(),
            params.fraction_dual_shopper_households,
            rng,
        );

        let mut per_household_states = (0..map.households.len())
            .map(|idx| {
                HouseholdState {
                    head_of_household_idx: 0, // To be filled in.
                    dual_shopper: dual_shopper_households[idx],
                    shopping_period_ticks: params.shopping_period_ticks,
                    supplies_bought_per_trip: params.supplies_bought_per_trip,
                    supply_levels: rng
                        .gen_range(params.init_supply_low_range, params.init_supply_high_range),
                }
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
            per_person_states,
            per_household_states,
        }
    }

    fn get_linear_path(source: &Position, dest: &Position) -> Vec<(u16, u16)> {
        let mut result = vec![source.clone()];

        let mut dx = dest.x - source.x;
        let mut dy = dest.y - source.y;
        let norm = (dx * dx + dy * dy).sqrt();

        dx /= norm;
        dy /= norm;

        loop {
            let head = &result[result.len() - 1];
            if head.distance(&dest) <= 1.0 {
                break;
            }
            let pos = Position {
                x: head.x + dx,
                y: head.y + dy,
            };
            if pos.x as i32 == dest.x as i32 && pos.y as i32 == dest.y as i32 {
                break;
            }
            result.push(pos);
        }

        result
            .into_iter()
            .map(|pos| (pos.y as u16, pos.x as u16))
            .collect::<Vec<_>>()
    }

    fn find_path_to_store(
        starting: &Position,
        household_idx: usize,
        map: &maps::Map,
        rng: &mut dyn RngCore,
    ) -> Result<Vec<(u16, u16)>> {
        let road_path = map.get_household_to_store_path(household_idx, 0, rng)?;

        // Add intermediate nodes from the starting position to the starting intersection.
        let mut entire_path = Self::get_linear_path(
            starting,
            &Position {
                x: road_path[0].1 as f32,
                y: road_path[0].0 as f32,
            },
        );
        entire_path.extend(road_path.into_iter());
        Ok(entire_path)
    }

    fn find_path_to_home(
        starting: &Position,
        household_idx: usize,
        map: &maps::Map,
        rng: &mut dyn RngCore,
    ) -> Result<Vec<(u16, u16)>> {
        let road_path = map.get_store_to_household_path(0, household_idx, rng)?;

        // Add intermediate nodes from the starting position to the starting intersection.
        let mut entire_path = Self::get_linear_path(
            starting,
            &Position {
                x: road_path[0].1 as f32,
                y: road_path[0].0 as f32,
            },
        );
        entire_path.extend(road_path.into_iter());
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
        //     If they are the head of household, calculate a path towards the store and update
        //       state to GoingToStore.
        //     If they are in a 2x-shopper household, then
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
        //   Case FollowHeadOfHousehold:
        for idx in 0..people.len() {
            let (left_people, right_people) = people.split_at_mut(idx);
            let person = &mut right_people[0];

            let (left_people_states, right_people_states) =
                self.per_person_states.split_at_mut(idx);
            let state = &mut right_people_states[0];

            let household = &map.households[person.household_idx];
            let household_state = &mut self.per_household_states[person.household_idx];

            match state {
                ShopperState::AtHome { direction_rad } => {
                    if household_state.supply_levels > 0.0 {
                        // Household supply levels are acceptable, brownian motion within household
                        person.position.advance(direction_rad, &household.bounds);
                        continue;
                    }

                    if person.head_of_household {
                        let path = Self::find_path_to_store(
                            &person.position,
                            person.household_idx,
                            map,
                            rng,
                        )
                        .expect("failed to find path");
                        *state = ShopperState::GoingToStore { path_idx: 0, path };
                        continue;
                    } else if household_state.dual_shopper {
                        *state = ShopperState::FollowHeadOfHousehold;
                        continue;
                    }

                    // This household only has a single shopper (head of household).
                    // We are not the head, so do not shop.
                    person.position.advance(direction_rad, &household.bounds);
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
                            shopping_duration_ticks: 0,
                        };
                    }
                }
                ShopperState::Shopping {
                    direction_rad,
                    shopping_duration_ticks,
                } => {
                    if *shopping_duration_ticks < household_state.shopping_period_ticks {
                        *shopping_duration_ticks += 1;
                        person
                            .position
                            .advance(direction_rad, &map.stores[0].bounds);
                    } else {
                        let path = Self::find_path_to_home(
                            &person.position,
                            person.household_idx,
                            map,
                            rng,
                        )
                        .expect("failed to find path");
                        *state = ShopperState::ReturningHome { path_idx: 0, path };
                    }
                }
                ShopperState::ReturningHome { path_idx, path } => {
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

                        household_state.supply_levels += household_state.supplies_bought_per_trip;

                        *state = ShopperState::AtHome {
                            direction_rad: rng.gen_range(0.0, 2.0 * PI),
                        };
                    }
                }
                ShopperState::FollowHeadOfHousehold => {
                    // If the head of household is at home and we are in the household, then
                    // transition to AtHome.
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

    fn update_household_state(&self, idx: usize, state: &mut wasm_view::HouseholdState) {
        let hs = &self.per_household_states[idx];

        state.dual_shopper = hs.dual_shopper;
        state.supply_levels = hs.supply_levels;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v0::maps::simple_groceries;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_find_path() -> Result<()> {
        let mut rng = Box::new(ChaCha8Rng::seed_from_u64(10914));
        let sg_map = maps::Map::load_from_ascii_str(simple_groceries::MAP_ASCII_STR, 10, 1)?;

        let household_idx = 19;
        assert_eq!(
            sg_map.households[household_idx].bounds,
            BoundingBox {
                bottom: 110,
                left: 60,
                top: 140,
                right: 90,
            }
        );

        let path = ShopperBehavior::find_path_to_store(
            &Position { x: 65.0, y: 125.0 },
            household_idx,
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
                        _ => panic!(
                            "invalid change in position in household: {} {}",
                            d_row, d_col
                        ),
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
