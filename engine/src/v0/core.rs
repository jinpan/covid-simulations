// Contains the core implementation of the v0 engine.

use crate::v0::config::*;
use crate::v0::disease_spread::{
    BackgroundViralParticleDiseaseSpreader, DiseaseSpreader, InfectionRadiusDiseaseSpreader,
};
use crate::v0::geometry::{Position, PositionAndDirection};
use crate::v0::maps;
use rand::{Rng, RngCore};
use std::f32::consts::PI;

#[derive(PartialEq, Debug)]
pub(crate) enum DiseaseState {
    // TODO: update this to a SEIR model
    Susceptible,
    // Exposed(usize), // Tick of when the person was exposed
    Infectious(usize), // Tick of when the person was infected
    Recovered,
}

pub(crate) struct Person {
    pub(crate) disease_state: DiseaseState,
    pub(crate) position_and_direction: PositionAndDirection,
}

impl Person {
    fn try_infect(&mut self, tick: usize) {
        match self.disease_state {
            DiseaseState::Susceptible => self.disease_state = DiseaseState::Infectious(tick),
            DiseaseState::Infectious(_) => {}
            DiseaseState::Recovered => {
                // For now, assume a simple SIR model where recovered cannot transition back to
                // Susceptible.
            }
        }
    }

    fn distance(&self, other: &Person) -> f32 {
        let p1 = &self.position_and_direction.position;
        let p2 = &other.position_and_direction.position;

        p1.distance(p2)
    }

    // Advance the position of the person by 1, and bounce off of the boundaries
    // of the world.
    fn update_position(&mut self, world_size: (u16, u16)) {
        self.position_and_direction.advance(world_size);
    }
}

pub(crate) struct World {
    pub(crate) config: WorldConfig,
    map: Option<maps::Map>,
    tick: usize,

    pub(crate) people: Vec<Person>,

    // TODO: refactor for static dispatch
    pub(crate) disease_spreader: Box<dyn DiseaseSpreader>,

    rng: Box<dyn RngCore>,
}

impl World {
    pub(crate) fn new(
        mut rng: Box<dyn RngCore>,
        config: WorldConfig,
        map: Option<maps::Map>,
    ) -> Self {
        assert!(config.num_people >= config.num_initially_infected);

        let people = (0..config.num_people)
            .map(|i| {
                let disease_state = if i < config.num_initially_infected {
                    DiseaseState::Infectious(0)
                } else {
                    DiseaseState::Susceptible
                };

                let x = rng.gen_range(0.0, config.size.0 as f32);
                let y = rng.gen_range(0.0, config.size.1 as f32);

                let direction_rad = rng.gen_range(0.0, 2.0 * PI);

                Person {
                    disease_state,
                    position_and_direction: PositionAndDirection {
                        position: Position { x, y },
                        direction_rad,
                    },
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

        World {
            config,
            map,
            tick: 0,
            people,
            disease_spreader,
            rng,
        }
    }

    pub fn step(&mut self) {
        self.tick += 1;
        let tick = self.tick;

        // Step 1: advance all the people
        let size = self.config.size;
        self.people.iter_mut().for_each(|p| {
            p.update_position(size);
        });

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
        expected_disease_states[2] = DiseaseState::Infectious(1);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[4] = DiseaseState::Infectious(5);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[0] = DiseaseState::Recovered;
        expected_disease_states[1] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[2] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[4] = DiseaseState::Recovered;
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
                        exhale_radius: 3.0,
                        exhale_amount: 1.0,
                        decay_rate: 0.5,
                        inhale_radius: 3.0,
                        infection_risk_per_particle: 0.5,
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
        expected_disease_states[2] = DiseaseState::Infectious(2);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[4] = DiseaseState::Infectious(5);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[0] = DiseaseState::Recovered;
        expected_disease_states[1] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[3] = DiseaseState::Infectious(7);
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[2] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[4] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        check_disease_states(&world.people, &expected_disease_states);

        world.step();
        expected_disease_states[3] = DiseaseState::Recovered;
        check_disease_states(&world.people, &expected_disease_states);
    }
}
