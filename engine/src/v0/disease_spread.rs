// Utilities for simulating the spread of disease.

use crate::rand::Rng;
use crate::v0::config::BackgroundViralParticleParams;
use crate::v0::core::{DiseaseState, Person};
use crate::v0::geometry::{BoundingBox, Position};
use crate::v0::types::Mask;
use rand_core::RngCore;

pub(crate) trait DiseaseSpreader {
    fn spread(&mut self, tick: usize, rng: &mut dyn RngCore, people: &mut [Person]);

    fn get_background_viral_levels(&self) -> &Vec<f32> {
        unimplemented!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Infection Radius Spread
///////////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////////
// Background Viral Particle Spread
///////////////////////////////////////////////////////////////////////////////

pub(crate) struct BackgroundViralParticleDiseaseSpreader {
    world_bounding_box: BoundingBox,
    params: BackgroundViralParticleParams,
    background_viral_particles: Vec<f32>,

    // Vec of (x, y) offsets that are in the exhale radius
    cells_in_exhale_radius: Vec<(i32, i32)>,
}

impl BackgroundViralParticleDiseaseSpreader {
    pub(crate) fn new(world_bb: BoundingBox, params: BackgroundViralParticleParams) -> Self {
        let background_viral_particles = vec![0.0; world_bb.size()];

        let mut cells_in_exhale_radius = vec![];
        for x in -params.exhale_radius as i32..params.exhale_radius as i32 {
            for y in -params.exhale_radius as i32..params.exhale_radius as i32 {
                if x * x + y * y <= (params.exhale_radius * params.exhale_radius) as i32 {
                    cells_in_exhale_radius.push((x, y));
                }
            }
        }

        BackgroundViralParticleDiseaseSpreader {
            world_bounding_box: world_bb,
            params,
            background_viral_particles,
            cells_in_exhale_radius,
        }
    }
}

impl BackgroundViralParticleDiseaseSpreader {
    fn get_particles_at(&self, x: usize, y: usize) -> f32 {
        let idx = self.world_bounding_box.right * y + x;
        self.background_viral_particles[idx]
    }

    fn mut_particles_at(particles: &mut Vec<f32>, width: usize, x: usize, y: usize) -> &mut f32 {
        let idx = width * y + x;
        &mut particles[idx]
    }

    fn update_background(&mut self, pos: &Position, particles_exhaled: f32) {
        let left = self.world_bounding_box.left as f32;
        let right = self.world_bounding_box.right as f32;

        let top = self.world_bounding_box.top as f32;
        let bottom = self.world_bounding_box.bottom as f32;

        for (dx, dy) in self.cells_in_exhale_radius.iter() {
            let x = pos.x + *dx as f32;
            if x < left || x >= right {
                continue;
            }
            let y = pos.y + *dy as f32;
            if y < bottom || y >= top {
                continue;
            }

            *Self::mut_particles_at(
                &mut self.background_viral_particles,
                self.world_bounding_box.right,
                x as usize,
                y as usize,
            ) += particles_exhaled;
        }
    }

    fn decay_existing_particles(&mut self) {
        let viral_particle_survival_rate = 1.0 - self.params.decay_rate;
        for val in self.background_viral_particles.iter_mut() {
            *val *= viral_particle_survival_rate;
        }
    }

    fn expose_susceptible_people(&self, tick: usize, rng: &mut dyn RngCore, people: &mut [Person]) {
        for p in people.iter_mut() {
            if let DiseaseState::Susceptible = p.disease_state {
            } else {
                continue;
            }

            let particles = self.get_particles_at(p.position.x as usize, p.position.y as usize);
            let particles_inhaled = match p.mask {
                Mask::N95 => particles / 5.0,
                _ => particles,
            };
            if particles_inhaled <= f32::MIN_POSITIVE {
                continue;
            }

            let infection_risk = particles_inhaled * self.params.infection_risk_per_particle;

            if rng.gen::<f32>() > infection_risk {
                continue;
            }
            p.disease_state = DiseaseState::Exposed(tick);
        }
    }

    fn infectious_people_exhale(&mut self, people: &mut [Person]) {
        for p in people.iter_mut() {
            if let DiseaseState::Infectious(_) = p.disease_state {
            } else {
                continue;
            }

            let particles_exhaled = match p.mask {
                Mask::N95 => 0.2,
                Mask::Regular => 0.2,
                Mask::None => 1.0,
            };

            self.update_background(&p.position, particles_exhaled);
        }
    }
}

impl DiseaseSpreader for BackgroundViralParticleDiseaseSpreader {
    fn spread(&mut self, tick: usize, rng: &mut dyn RngCore, people: &mut [Person]) {
        // Step 1: Decay the existing particles
        self.decay_existing_particles();

        // Step 2: All people inhale, and may become exposed according to how much they have
        // inhaled.
        self.expose_susceptible_people(tick, rng, people);

        // Step 3: All people exhale, and infected people spread viral particles.
        self.infectious_people_exhale(people);
    }

    fn get_background_viral_levels(&self) -> &Vec<f32> {
        &self.background_viral_particles
    }
}
