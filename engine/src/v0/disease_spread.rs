// Utilities for simulating the spread of disease.

use crate::rand::Rng;
use crate::v0::config::BackgroundViralParticleParams;
use crate::v0::core::{DiseaseState, Person};
use crate::v0::geometry::{BoundingBox, Position};
use rand_core::RngCore;

pub(crate) trait DiseaseSpreader {
    fn spread(&mut self, tick: usize, rng: &mut dyn RngCore, people: &mut [Person]);

    fn get_background_viral_levels(&self) -> &Vec<Vec<f32>> {
        unimplemented!()
    }
}

pub(crate) struct InfectionRadiusDiseaseSpreader {
    radius: f32,
}

impl InfectionRadiusDiseaseSpreader {
    pub(crate) fn new(radius: f32) -> Self {
        InfectionRadiusDiseaseSpreader { radius }
    }
}

impl DiseaseSpreader for InfectionRadiusDiseaseSpreader {
    fn spread(&mut self, tick: usize, _: &mut dyn RngCore, people: &mut [Person]) {
        // TODO: instead of a N^2 loop, use some index structure (BTreeMap?)
        for i in 0..(people.len() - 1) {
            let (left, right) = people.split_at_mut(i + 1);
            let p0 = left.last_mut().unwrap();
            let p0_position = &p0.position;

            for p1 in right.iter_mut() {
                let p1_position = &p1.position;

                if p0_position.distance(p1_position) >= self.radius {
                    continue;
                }

                match (&p0.disease_state, &p1.disease_state) {
                    // If one person is susceptible and the other is infectious, expose the
                    // susceptible person.
                    (DiseaseState::Susceptible, DiseaseState::Infectious(_)) => {
                        p0.disease_state = DiseaseState::Exposed(tick);
                    }
                    (DiseaseState::Infectious(_), DiseaseState::Susceptible) => {
                        p1.disease_state = DiseaseState::Exposed(tick);
                    }
                    // Otherwise, no-op.
                    (DiseaseState::Susceptible, DiseaseState::Susceptible) => (),
                    (DiseaseState::Recovered, _) | (_, DiseaseState::Recovered) => (),
                    (DiseaseState::Exposed(_), _) | (_, DiseaseState::Exposed(_)) => (),
                    (DiseaseState::Infectious(_), DiseaseState::Infectious(_)) => (),
                };
            }
        }
    }
}

pub(crate) struct BackgroundViralParticleDiseaseSpreader {
    world_bounding_box: BoundingBox,
    params: BackgroundViralParticleParams,
    background_viral_particles: Vec<Vec<f32>>,
}

impl BackgroundViralParticleDiseaseSpreader {
    pub(crate) fn new(world_bb: BoundingBox, params: BackgroundViralParticleParams) -> Self {
        let background_viral_particles = vec![vec![0.0; world_bb.right]; world_bb.top];

        BackgroundViralParticleDiseaseSpreader {
            world_bounding_box: world_bb,
            params,
            background_viral_particles,
        }
    }
}

impl BackgroundViralParticleDiseaseSpreader {
    // Helper function for getting cells within a radius of the position
    fn get_cells(pos: &Position, radius: f32, world_bb: &BoundingBox) -> Vec<(u16, u16)> {
        let min_x = f32::max(0.0, pos.x - radius).round() as u16;
        let max_x = f32::min(world_bb.right as f32 - 1.0, pos.x + radius).round() as u16;

        let min_y = f32::max(0.0, pos.y - radius).round() as u16;
        let max_y = f32::min(world_bb.top as f32 - 1.0, pos.y + radius).round() as u16;

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
}

impl DiseaseSpreader for BackgroundViralParticleDiseaseSpreader {
    fn spread(&mut self, tick: usize, rng: &mut dyn RngCore, people: &mut [Person]) {
        let params = &self.params;
        let background_viral_particles = &mut self.background_viral_particles;

        // Step 1: Decay the existing particles
        let viral_particle_survival_rate = 1.0 - params.decay_rate;
        for row in background_viral_particles.iter_mut() {
            for val in row.iter_mut() {
                if *val > f32::MIN_POSITIVE {
                    // Without this branch, performance slows down significantly in the browser.
                    // Unclear why.
                    *val *= viral_particle_survival_rate;
                }
            }
        }

        // Step 2: All people inhale, and may become exposed according to how much they have
        // inhaled.
        for p in people.iter_mut() {
            if let DiseaseState::Susceptible = p.disease_state {
            } else {
                continue;
            }

            let particles_inhaled =
                background_viral_particles[p.position.y as usize][p.position.x as usize] as f32;
            if particles_inhaled <= f32::MIN_POSITIVE {
                continue;
            }

            let infection_risk = particles_inhaled * params.infection_risk_per_particle;

            if rng.gen::<f32>() > infection_risk {
                continue;
            }
            p.disease_state = DiseaseState::Exposed(tick);
        }

        // Step 3: All people exhale, and infected people spread viral particles.
        for p in people.iter_mut() {
            if let DiseaseState::Infectious(_) = p.disease_state {
            } else {
                continue;
            }

            Self::get_cells(&p.position, params.exhale_radius, &self.world_bounding_box)
                .iter()
                .for_each(|(x, y)| {
                    background_viral_particles[*y as usize][*x as usize] += 1.0;
                });
        }
    }

    fn get_background_viral_levels(&self) -> &Vec<Vec<f32>> {
        &self.background_viral_particles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cells() {
        let world_bb = BoundingBox {
            bottom: 0,
            left: 0,
            top: 10,
            right: 10,
        };
        let cells = BackgroundViralParticleDiseaseSpreader::get_cells(
            &Position { x: 5.0, y: 5.0 },
            0.5,
            &world_bb,
        );
        assert_eq!(cells, vec![(5, 5)]);

        let mut cells = BackgroundViralParticleDiseaseSpreader::get_cells(
            &Position { x: 5.0, y: 5.0 },
            1.0,
            &world_bb,
        );
        cells.sort();
        assert_eq!(cells, vec![(4, 5), (5, 4), (5, 5), (5, 6), (6, 5)]);

        // (0, 0)
        let mut cells = BackgroundViralParticleDiseaseSpreader::get_cells(
            &Position { x: 0.0, y: 0.0 },
            1.0,
            &world_bb,
        );
        cells.sort();
        assert_eq!(cells, vec![(0, 0), (0, 1), (1, 0)]);

        // (0, 10)
        let mut cells = BackgroundViralParticleDiseaseSpreader::get_cells(
            &Position { x: 0.0, y: 9.0 },
            1.0,
            &world_bb,
        );
        cells.sort();
        assert_eq!(cells, vec![(0, 8), (0, 9), (1, 9)]);

        // (10, 0)
        let mut cells = BackgroundViralParticleDiseaseSpreader::get_cells(
            &Position { x: 9.0, y: 0.0 },
            1.0,
            &world_bb,
        );
        cells.sort();
        assert_eq!(cells, vec![(8, 0), (9, 0), (9, 1)]);

        // (10, 10)
        let mut cells = BackgroundViralParticleDiseaseSpreader::get_cells(
            &Position { x: 9.0, y: 9.0 },
            1.0,
            &world_bb,
        );
        cells.sort();
        assert_eq!(cells, vec![(8, 9), (9, 8), (9, 9)]);
    }
}
