use crate::v0::core::Person;
use crate::v0::geometry::BoundingBox;
use crate::v0::maps;
use rand::{Rng, RngCore};
use std::f32::consts::PI;
use std::iter::Iterator;

pub(crate) trait PersonBehavior {
    fn update_positions(&mut self, people: &mut [Person], map: &mut Option<maps::Map>);
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
    fn update_positions(&mut self, people: &mut [Person], _: &mut Option<maps::Map>) {
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
    AtHome { direction_rad: f32 },
    GoingToStore,
    Shopping,
    ReturningHome,
}

// Shoppers are in
pub(crate) struct ShopperBehavior {
    world_size: (u16, u16),
    per_person_states: Vec<ShopperState>,
}

impl ShopperBehavior {
    pub(crate) fn new(world_size: (u16, u16), num_people: usize, rng: &mut dyn RngCore) -> Self {
        let per_person_states = (0..num_people)
            .map(|_| ShopperState::AtHome {
                direction_rad: rng.gen_range(0.0, 2.0 * PI),
            })
            .collect();

        ShopperBehavior {
            world_size,
            per_person_states,
        }
    }

    fn find_bb_road_intersection(
        world_size: (u16, u16),
        household_bb: &BoundingBox,
        map: maps::Map,
    ) -> Vec<((u16, u16), (u16, u16))> {
        // Returns a list of a pair of points, such that
        //   the first point of the pair is inside the household
        //   the second point of the pair is on the road
        //   the first and second points are adjacent
        let mut intersections = vec![];

        // Iterate over the top boundary of the bounding box.
        if household_bb.top_left.0 > 0 {
            let row = household_bb.top_left.0 - 1;
            for col in household_bb.cols() {
                if map.elements[row][col] == maps::MapElement::Road {
                    intersections.push((
                        (household_bb.top_left.0 as u16, col as u16),
                        (row as u16, col as u16),
                    ));
                }
            }
        }

        // Iterate over the left boundary of the bounding box.
        if household_bb.top_left.1 > 0 {
            let col = household_bb.top_left.1 - 1;
            for row in household_bb.rows() {
                if map.elements[row][col] == maps::MapElement::Road {
                    intersections.push((
                        (row as u16, household_bb.top_left.1 as u16),
                        (row as u16, col as u16),
                    ));
                }
            }
        }

        // Iterate over the bottom boundary of the bounding box
        if (household_bb.bottom_right.0 as u16) < world_size.1 {
            let row = household_bb.bottom_right.0;
            for col in household_bb.cols() {
                if map.elements[row][col] == maps::MapElement::Road {
                    intersections.push((
                        ((household_bb.bottom_right.0 - 1) as u16, col as u16),
                        (row as u16, col as u16),
                    ));
                }
            }
        }

        // Iterate over the right boundary of the bounding box.
        if (household_bb.bottom_right.1 as u16) < world_size.0 {
            let col = household_bb.bottom_right.1;
            for row in household_bb.rows() {
                if map.elements[row][col] == maps::MapElement::Road {
                    intersections.push((
                        (row as u16, (household_bb.bottom_right.1 - 1) as u16),
                        (row as u16, col as u16),
                    ));
                }
            }
        }

        intersections
    }

    fn find_path(
        from_bb: &BoundingBox,
        to_bb: &BoundingBox,
        map: maps::Map,
    ) -> Option<Vec<(u16, u16)>> {
        None
    }
}

impl PersonBehavior for ShopperBehavior {
    fn update_positions(&mut self, people: &mut [Person], maybe_map: &mut Option<maps::Map>) {
        // Step 0: Update the household supply levels.
        let map = maybe_map
            .as_mut()
            .expect("shopper behavior must have a map");
        for household in map.households.iter_mut() {
            household.supply_levels -= 1.0;
        }

        // TODO: augment these steps for multi-person households.

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
        for (idx, person) in people.iter_mut().enumerate() {
            let state = &mut self.per_person_states[idx];
            match state {
                ShopperState::AtHome { direction_rad } => {
                    let household = &map.households[person.household_idx];
                    if household.supply_levels > 0.0 {
                        // Household supply levels are acceptable, brownian motion within household
                        person.position.advance2(direction_rad, &household.bounds);
                        continue;
                    }
                    // TODO: Calculate a path towards the store and update state to GoingToStore.
                }
                ShopperState::GoingToStore => unimplemented!("TODO"),
                ShopperState::Shopping => unimplemented!("TODO"),
                ShopperState::ReturningHome => unimplemented!("TODO"),
            }
        }
    }
}
