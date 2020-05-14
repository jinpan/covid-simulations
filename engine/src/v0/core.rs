// Contains the core implementation of the v0 engine.

use crate::v0::config::*;
use crate::v0::disease_spread::{
    BackgroundViralParticleDiseaseSpreader, DiseaseSpreader, InfectionRadiusDiseaseSpreader,
};
use crate::v0::geometry::Position;
use crate::v0::maps;
use crate::v0::person_behavior::{BrownianMotionBehavior, PersonBehavior, ShopperBehavior};
use rand::{Rng, RngCore};

#[derive(PartialEq, Debug)]
pub(crate) enum DiseaseState {
    // TODO: update this to a SEIR model
    Susceptible,
    // Exposed(usize), // Tick of when the person was exposed
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
    tick: usize,

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

        let mut current_household_idx = 0;
        let mut people_in_current_household = 0;
        let people = (0..config.num_people)
            .map(|i| {
                let disease_state = if i < config.num_initially_infected {
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
                    x = rng.gen_range(
                        household.bounds.top_left.1 as f32,
                        household.bounds.bottom_right.1 as f32,
                    );
                    y = rng.gen_range(
                        household.bounds.top_left.0 as f32,
                        household.bounds.bottom_right.0 as f32,
                    );
                } else {
                    x = rng.gen_range(0.0, config.size.0 as f32);
                    y = rng.gen_range(0.0, config.size.1 as f32);
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
                    BackgroundViralParticleDiseaseSpreader::new(config.size, params),
                ),
            };

        let person_behavior: Box<dyn PersonBehavior> = match config.behavior_parameters {
            BehaviorParameters::BrownianMotion => Box::new(BrownianMotionBehavior::new(
                config.size,
                config.num_people,
                &mut rng,
            )),
            BehaviorParameters::Shopper => Box::new(ShopperBehavior::new(
                config.size,
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

        // Step 2: advance infectious states to recovered
        let disease_parameters = &self.config.disease_parameters;
        self.people.iter_mut().for_each(|p| {
            if let DiseaseState::Infectious(start_tick) = p.disease_state {
                if tick - start_tick > disease_parameters.infectious_period_ticks {
                    p.disease_state = DiseaseState::Recovered;
                }
            }
        });

        // Step 3: identify collisions and update disease state.
        self.disease_spreader
            .spread(tick, &mut self.rng, &mut self.people);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
                infectious_period_ticks: 5,
                spread_parameters: DiseaseSpreadParameters::InfectionRadius(5.0),
            },
            behavior_parameters: BehaviorParameters::BrownianMotion,
            size: (10, 10),
            num_people: 5,
            num_initially_infected: 2,
        };

        let mut world = World::new(rng, config, None);
        let mut expected_disease_states = vec![
            DiseaseState::Infectious(0),
            DiseaseState::Infectious(0),
            DiseaseState::Susceptible,
            DiseaseState::Susceptible,
            DiseaseState::Susceptible,
        ];

        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[3] = DiseaseState::Infectious(1);
        expected_disease_states[4] = DiseaseState::Infectious(1);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[2] = DiseaseState::Infectious(4);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[0] = DiseaseState::Recovered;
        expected_disease_states[1] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[3] = DiseaseState::Recovered;
        expected_disease_states[4] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[2] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);
    }

    #[test]
    fn test_viral_particle_spread_world() {
        let rng = Box::new(ChaCha8Rng::seed_from_u64(10914));
        let config = WorldConfig {
            disease_parameters: DiseaseParameters {
                infectious_period_ticks: 5,
                spread_parameters: DiseaseSpreadParameters::BackgroundViralParticle(
                    BackgroundViralParticleParams {
                        exhale_radius: 4.0,
                        exhale_amount: 1.0,
                        decay_rate: 0.5,
                        infection_risk_per_particle: 0.8,
                    },
                ),
            },
            behavior_parameters: BehaviorParameters::BrownianMotion,
            size: (10, 10),
            num_people: 5,
            num_initially_infected: 2,
        };

        let mut world = World::new(rng, config, None);
        let mut expected_disease_states = vec![
            DiseaseState::Infectious(0),
            DiseaseState::Infectious(0),
            DiseaseState::Susceptible,
            DiseaseState::Susceptible,
            DiseaseState::Susceptible,
        ];

        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);
        world.step();
        expected_disease_states[3] = DiseaseState::Infectious(2);
        expected_disease_states[4] = DiseaseState::Infectious(2);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);
        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[2] = DiseaseState::Infectious(5);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[0] = DiseaseState::Recovered;
        expected_disease_states[1] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[3] = DiseaseState::Recovered;
        expected_disease_states[4] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);
        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[2] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);
    }
}
