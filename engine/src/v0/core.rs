// Contains the core implementation of the v0 engine.

use crate::v0::config::*;
use crate::v0::disease_spread::{
    BackgroundViralParticleDiseaseSpreader, DiseaseSpreader, InfectionRadiusDiseaseSpreader,
};
use crate::v0::geometry::Position;
use crate::v0::maps;
use crate::v0::person_behavior::{BrownianMotionBehavior, PersonBehavior, ShopperBehavior};
use rand::prelude::SliceRandom;
use rand::{Rng, RngCore};

#[derive(PartialEq, Debug)]
pub(crate) enum DiseaseState {
    Susceptible,
    Exposed(usize),    // Tick of when the person was exposed
    Infectious(usize), // Tick of when the person was infected
    Recovered,
}

pub(crate) struct Person {
    pub(crate) id: usize,
    pub(crate) disease_state: DiseaseState,
    pub(crate) position: Position,

    // Index into the map's household for the person if the map exists.
    // Otherwise, 0.
    pub(crate) household_idx: usize,
    pub(crate) head_of_household: bool,
}

pub(crate) struct World {
    pub(crate) config: WorldConfig,
    pub(crate) map: Option<maps::Map>,
    pub(crate) tick: usize,

    pub(crate) people: Vec<Person>,

    // TODO: refactor for static dispatch
    pub(crate) disease_spreader: Box<dyn DiseaseSpreader>,
    pub(crate) person_behavior: Box<dyn PersonBehavior>,

    rng: Box<dyn RngCore>,
}

impl World {
    pub(crate) fn new(
        mut rng: Box<dyn RngCore>,
        config: WorldConfig,
        maybe_map: Option<maps::Map>,
    ) -> Self {
        assert!(config.num_people >= config.num_initially_infected);

        let mut infected_people = vec![false; config.num_people];
        for i in 0..config.num_initially_infected {
            infected_people[i] = true;
        }
        infected_people.shuffle(&mut rng);

        let mut current_household_idx = 0;
        let mut people_in_current_household = 0;
        let people = (0..config.num_people)
            .map(|i| {
                let disease_state = if infected_people[i] {
                    DiseaseState::Infectious(0)
                } else {
                    DiseaseState::Susceptible
                };

                let x: f32;
                let y: f32;

                if let Some(map) = &maybe_map {
                    let household = &map.households[current_household_idx];
                    if people_in_current_household == household.num_people {
                        current_household_idx += 1;
                        people_in_current_household = 0;
                    }
                    people_in_current_household += 1;

                    let household = &map.households[current_household_idx];
                    x = rng.gen_range(household.bounds.left as f32, household.bounds.right as f32);
                    y = rng.gen_range(household.bounds.top as f32, household.bounds.bottom as f32);
                } else {
                    x = rng.gen_range(0.0, config.bounding_box.right as f32);
                    y = rng.gen_range(0.0, config.bounding_box.bottom as f32);
                }

                Person {
                    id: i,
                    disease_state,
                    position: Position { x, y },
                    household_idx: current_household_idx,
                    head_of_household: people_in_current_household == 1,
                }
            })
            .collect();

        let disease_spreader: Box<dyn DiseaseSpreader> =
            match config.disease_parameters.spread_parameters {
                DiseaseSpreadParameters::InfectionRadius(r) => {
                    Box::new(InfectionRadiusDiseaseSpreader::new(r))
                }

                DiseaseSpreadParameters::BackgroundViralParticle(params) => Box::new(
                    BackgroundViralParticleDiseaseSpreader::new(config.bounding_box, params),
                ),
            };

        let person_behavior: Box<dyn PersonBehavior> = match config.behavior_parameters {
            BehaviorParameters::BrownianMotion => Box::new(BrownianMotionBehavior::new(
                config.bounding_box,
                config.num_people,
                &mut rng,
            )),
            BehaviorParameters::Shopper(params) => Box::new(ShopperBehavior::new(
                config.bounding_box,
                params,
                &people,
                maybe_map
                    .as_ref()
                    .expect("must have map for shopper behavior"),
                &mut rng,
            )),
        };

        World {
            config,
            map: maybe_map,
            tick: 0,
            people,
            disease_spreader,
            person_behavior,
            rng,
        }
    }

    pub fn step(&mut self) {
        self.tick += 1;
        let tick = self.tick;

        // Step 1: advance all the people
        self.person_behavior
            .update_positions(&mut self.people, &mut self.map, &mut self.rng);

        // Step 2: Update disease state according to the spread model.
        self.disease_spreader
            .spread(tick, &mut self.rng, &mut self.people);

        // Step 3: Update time-based disease states:
        //   * Advance exposed states to infectious
        //   * Advance infectious states to recovered
        let disease_parameters = &self.config.disease_parameters;
        self.people.iter_mut().for_each(|p| match p.disease_state {
            DiseaseState::Exposed(start_tick) => {
                if tick - start_tick >= disease_parameters.exposed_period_ticks {
                    p.disease_state = DiseaseState::Infectious(tick);
                }
            }
            DiseaseState::Infectious(start_tick) => {
                if tick - start_tick >= disease_parameters.infectious_period_ticks {
                    p.disease_state = DiseaseState::Recovered;
                }
            }
            DiseaseState::Susceptible | DiseaseState::Recovered => {}
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v0::geometry::BoundingBox;
    use itertools::Itertools;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn check_disease_states(people: &[Person], expected: &[DiseaseState]) {
        for (idx, (person, state)) in people.iter().zip_eq(expected.iter()).enumerate() {
            assert_eq!(person.disease_state, *state, "diff at index {}", idx);
        }
    }

    #[test]
    fn test_infection_radius_world() {
        let rng = Box::new(ChaCha8Rng::seed_from_u64(10914));
        let config = WorldConfig {
            disease_parameters: DiseaseParameters {
                exposed_period_ticks: 0,
                infectious_period_ticks: 5,
                spread_parameters: DiseaseSpreadParameters::InfectionRadius(5.0),
            },
            behavior_parameters: BehaviorParameters::BrownianMotion,
            bounding_box: BoundingBox {
                top: 0,
                left: 0,
                bottom: 10,
                right: 10,
            },
            num_people: 5,
            num_initially_infected: 2,
        };

        let mut world = World::new(rng, config, None);
        let mut expected_disease_states = vec![
            DiseaseState::Susceptible,
            DiseaseState::Susceptible,
            DiseaseState::Susceptible,
            DiseaseState::Infectious(0),
            DiseaseState::Infectious(0),
        ];

        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[2] = DiseaseState::Infectious(1);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[1] = DiseaseState::Infectious(2);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[0] = DiseaseState::Infectious(3);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[3] = DiseaseState::Recovered;
        expected_disease_states[4] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[2] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[1] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[0] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);
    }

    #[test]
    fn test_viral_particle_spread_world() {
        let rng = Box::new(ChaCha8Rng::seed_from_u64(10914));
        let config = WorldConfig {
            disease_parameters: DiseaseParameters {
                exposed_period_ticks: 1,
                infectious_period_ticks: 5,
                spread_parameters: DiseaseSpreadParameters::BackgroundViralParticle(
                    BackgroundViralParticleParams {
                        exhale_radius: 4.0,
                        decay_rate: 0.5,
                        infection_risk_per_particle: 0.8,
                    },
                ),
            },
            behavior_parameters: BehaviorParameters::BrownianMotion,
            bounding_box: BoundingBox {
                top: 0,
                left: 0,
                bottom: 10,
                right: 10,
            },
            num_people: 5,
            num_initially_infected: 2,
        };

        let mut world = World::new(rng, config, None);
        let mut expected_disease_states = vec![
            DiseaseState::Susceptible,
            DiseaseState::Susceptible,
            DiseaseState::Susceptible,
            DiseaseState::Infectious(0),
            DiseaseState::Infectious(0),
        ];

        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);
        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[1] = DiseaseState::Exposed(3);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[1] = DiseaseState::Infectious(4);
        expected_disease_states[2] = DiseaseState::Exposed(4);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[0] = DiseaseState::Exposed(5);
        expected_disease_states[2] = DiseaseState::Infectious(5);
        expected_disease_states[3] = DiseaseState::Recovered;
        expected_disease_states[4] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[0] = DiseaseState::Infectious(6);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);
        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[1] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[2] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[0] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);
    }
}
