// Contains the core implementation of the v0 engine.

use crate::v0::geometry::{Position, PositionAndDirection};
use rand::{Rng, RngCore};
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;
use std::f32::consts::PI;

#[derive(Deserialize, Debug)]
pub(crate) struct DiseaseParameters {
    infection_radius: u16,
    infectious_period_ticks: usize,
}

#[derive(Deserialize, Debug)]
pub(crate) struct WorldConfig {
    disease_parameters: DiseaseParameters,
    size: (u16, u16),
    num_people: usize,
    num_initially_infected: usize,
}

#[derive(PartialEq, Debug)]
pub(crate) enum DiseaseState {
    Susceptible,
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
    fn update_position(&mut self, world_size: &(u16, u16)) {
        self.position_and_direction.advance(world_size);
    }
}

pub(crate) struct World {
    config: WorldConfig,
    pub(crate) people: Vec<Person>,
    tick: usize,

    rng: Box<dyn RngCore>,
}

impl World {
    pub(crate) fn new(mut rng: Box<dyn RngCore>, config: WorldConfig) -> Self {
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

        World {
            config,
            people,
            tick: 0,
            rng,
        }
    }

    pub fn step(&mut self) {
        self.tick += 1;
        let tick = self.tick;

        // Step 1: advance all the people
        let size = &self.config.size;
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
        // TODO: instead of a N^2 loop, use some index structure (BTreeMap?)
        for i in 0..(self.people.len() - 1) {
            let (left, right) = self.people.split_at_mut(i + 1);
            let p0 = left.last_mut().unwrap();

            for p1 in right.iter_mut() {
                if p0.distance(p1) >= disease_parameters.infection_radius as f32 {
                    continue;
                }

                if let DiseaseState::Infectious(_) = p0.disease_state {
                    p1.try_infect(tick);
                }
                if let DiseaseState::Infectious(_) = p1.disease_state {
                    p0.try_infect(tick);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_world() {
        let rng = Box::new(ChaCha8Rng::seed_from_u64(10914));
        let config = WorldConfig {
            disease_parameters: DiseaseParameters {
                infection_radius: 5,
                infectious_period_ticks: 5,
            },
            size: (10, 10),
            num_people: 5,
            num_initially_infected: 2,
        };

        let mut world = World::new(rng, config);

        // Check that two people are infected.
        let num_infected = world
            .people
            .iter()
            .filter(|p| match p.disease_state {
                DiseaseState::Susceptible => false,
                DiseaseState::Infectious(n) => {
                    assert_eq!(n, 0);
                    true
                }
                DiseaseState::Recovered => panic!("people cannot start out as recovered"),
            })
            .count();
        assert_eq!(num_infected, 2);

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[1].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[2].disease_state, DiseaseState::Infectious(1));
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Susceptible);

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[1].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[2].disease_state, DiseaseState::Infectious(1));
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Susceptible);

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[1].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[2].disease_state, DiseaseState::Infectious(1));
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Susceptible);

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[1].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[2].disease_state, DiseaseState::Infectious(1));
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Susceptible);

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[1].disease_state, DiseaseState::Infectious(0));
        assert_eq!(world.people[2].disease_state, DiseaseState::Infectious(1));
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Infectious(5));

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[1].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[2].disease_state, DiseaseState::Infectious(1));
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Infectious(5));

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[1].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[2].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Infectious(5));

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[1].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[2].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Infectious(5));

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[1].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[2].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Infectious(5));

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[1].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[2].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Infectious(5));

        world.step();
        assert_eq!(world.people[0].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[1].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[2].disease_state, DiseaseState::Recovered);
        assert_eq!(world.people[3].disease_state, DiseaseState::Susceptible);
        assert_eq!(world.people[4].disease_state, DiseaseState::Recovered);
    }
}
