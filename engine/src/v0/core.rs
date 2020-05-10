// Contains the core implementation of the v0 engine.

use crate::v0::config::*;
use crate::v0::geometry::{Position, PositionAndDirection};
use rand::{Rng, RngCore};
use std::f32::consts::PI;

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
    fn update_position(&mut self, world_size: (u16, u16)) {
        self.position_and_direction.advance(world_size);
    }
}

pub(crate) struct World {
    pub(crate) config: WorldConfig,
    tick: usize,

    pub(crate) people: Vec<Person>,

    // TODO: don't create this map for worlds that don't require it.
    pub(crate) background_viral_particles: Vec<Vec<f32>>,

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

        let background_viral_particles =
            vec![vec![0.0; config.size.0 as usize]; config.size.1 as usize];

        World {
            config,
            tick: 0,
            people,
            background_viral_particles,
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
        self.spread_disease(tick);
    }

    fn spread_disease_infection_radius(tick: usize, radius: f32, people: &mut Vec<Person>) {
        // TODO: instead of a N^2 loop, use some index structure (BTreeMap?)
        for i in 0..(people.len() - 1) {
            let (left, right) = people.split_at_mut(i + 1);
            let p0 = left.last_mut().unwrap();

            for p1 in right.iter_mut() {
                if p0.distance(p1) >= radius {
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
    // Helper function for getting cells within a radius of the position
    fn get_cells(pos: &Position, radius: f32, world_size: (u16, u16)) -> Vec<(u16, u16)> {
        let min_x = f32::max(0.0, pos.x - radius).round() as u16;
        let max_x = f32::min(world_size.0 as f32 - 1.0, pos.x + radius).round() as u16;

        let min_y = f32::max(0.0, pos.y - radius).round() as u16;
        let max_y = f32::min(world_size.1 as f32 - 1.0, pos.y + radius).round() as u16;

        let mut cells = vec![];
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if pos.distance(&Position {
                    x: x as f32,
                    y: y as f32,
                }) <= radius
                {
                    cells.push((x, y));
                }
            }
        }
        cells
    }

    fn spread_disease_background_viral_particle(
        tick: usize,
        rng: &mut dyn RngCore,
        params: &BackgroundViralParticleParams,
        people: &mut Vec<Person>,
        background_viral_particles: &mut Vec<Vec<f32>>,
        world_size: (u16, u16),
    ) {
        // Step 1: Decay the existing particles
        let viral_particle_survival_rate = 1.0 - params.decay_rate;
        for row in background_viral_particles.iter_mut() {
            for val in row.iter_mut() {
                *val *= viral_particle_survival_rate;
            }
        }

        // Step 2: All people inhale, and may become infected according to how much they have
        // inhaled.
        for p in people.iter_mut() {
            if let DiseaseState::Susceptible = p.disease_state {
            } else {
                continue;
            }

            let particles_inhaled = World::get_cells(
                &p.position_and_direction.position,
                params.inhale_radius,
                world_size,
            )
            .iter()
            .map(|(x, y)| background_viral_particles[*y as usize][*x as usize])
            .sum::<f32>();
            let infection_risk = particles_inhaled * params.infection_risk_per_particle;

            if rng.gen::<f32>() > infection_risk {
                continue;
            }
            p.disease_state = DiseaseState::Infectious(tick);
        }

        // Step 3: All people exhale, and infected people spread viral particles.
        for p in people.iter_mut() {
            if let DiseaseState::Infectious(_) = p.disease_state {
            } else {
                continue;
            }

            World::get_cells(
                &p.position_and_direction.position,
                params.exhale_radius,
                world_size,
            )
            .iter()
            .for_each(|(x, y)| {
                background_viral_particles[*y as usize][*x as usize] += params.exhale_amount;
            });
        }
    }

    fn spread_disease(&mut self, tick: usize) {
        match &self.config.disease_parameters.spread_parameters {
            DiseaseSpreadParameters::InfectionRadius(r) => {
                World::spread_disease_infection_radius(tick, *r, &mut self.people)
            }
            DiseaseSpreadParameters::BackgroundViralParticle(params) => {
                World::spread_disease_background_viral_particle(
                    tick,
                    &mut self.rng,
                    params,
                    &mut self.people,
                    &mut self.background_viral_particles,
                    self.config.size,
                )
            }
        }
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
            size: (10, 10),
            num_people: 5,
            num_initially_infected: 2,
        };

        let mut world = World::new(rng, config);
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
            size: (10, 10),
            num_people: 5,
            num_initially_infected: 2,
        };

        let mut world = World::new(rng, config);
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

    #[test]
    fn test_world_get_cells() {
        let cells = World::get_cells(&Position { x: 5.0, y: 5.0 }, 0.5, (10, 10));
        assert_eq!(cells, vec![(5, 5)]);

        let mut cells = World::get_cells(&Position { x: 5.0, y: 5.0 }, 1.0, (10, 10));
        cells.sort();
        assert_eq!(cells, vec![(4, 5), (5, 4), (5, 5), (5, 6), (6, 5)]);

        // (0, 0)
        let mut cells = World::get_cells(&Position { x: 0.0, y: 0.0 }, 1.0, (10, 10));
        cells.sort();
        assert_eq!(cells, vec![(0, 0), (0, 1), (1, 0)]);

        // (0, 10)
        let mut cells = World::get_cells(&Position { x: 0.0, y: 9.0 }, 1.0, (10, 10));
        cells.sort();
        assert_eq!(cells, vec![(0, 8), (0, 9), (1, 9)]);

        // (10, 0)
        let mut cells = World::get_cells(&Position { x: 9.0, y: 0.0 }, 1.0, (10, 10));
        cells.sort();
        assert_eq!(cells, vec![(8, 0), (9, 0), (9, 1)]);

        // (10, 10)
        let mut cells = World::get_cells(&Position { x: 9.0, y: 9.0 }, 1.0, (10, 10));
        cells.sort();
        assert_eq!(cells, vec![(8, 9), (9, 8), (9, 9)]);
    }
}
