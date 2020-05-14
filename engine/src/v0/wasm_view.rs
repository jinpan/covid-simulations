// Provides wasm bindings for the v0 engine.
use wasm_bindgen::prelude::*;

use crate::v0::config::{DiseaseSpreadParameters, WorldConfig};
use crate::v0::core::{DiseaseState, World};
use crate::v0::geometry;
use crate::v0::maps;
use itertools::Itertools;
use rand::RngCore;

#[derive(Serialize)]
pub struct BoundingBox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bot: f32,
}

impl BoundingBox {
    fn from_geo(geo_box: &geometry::BoundingBox) -> Self {
        BoundingBox {
            left: geo_box.top_left.1 as f32,
            right: geo_box.bottom_right.1 as f32,
            top: geo_box.top_left.0 as f32,
            bot: geo_box.bottom_right.0 as f32,
        }
    }
}

#[derive(Serialize)]
pub struct Household {
    pub bounds: BoundingBox,
    pub dual_shopper: bool,
}

#[derive(Serialize)]
pub struct Person {
    pub id: usize,
    pub px: f32,
    pub py: f32,
    pub ds: String,
}

#[derive(Serialize)]
pub struct State {
    pub tick: usize,
    pub people: Vec<Person>,
}

#[wasm_bindgen]
pub struct WorldView {
    world: World,

    background_viral_particles: Vec<f32>,
}

impl WorldView {
    pub fn new(config: WorldConfig, map: Option<maps::Map>, rng: Box<dyn RngCore>) -> Self {
        let world = World::new(rng, config, map);

        let background_viral_particles =
            Vec::with_capacity(world.config.size.0 as usize * world.config.size.1 as usize);

        WorldView {
            world,
            background_viral_particles,
        }
    }
}

#[wasm_bindgen]
impl WorldView {
    pub fn step(&mut self) {
        self.world.step();
    }

    pub fn to_json(&self) -> JsValue {
        let people = self
            .world
            .people
            .iter()
            .map(|p| {
                let disease_state = match p.disease_state {
                    DiseaseState::Susceptible => "susceptible",
                    DiseaseState::Exposed(_) => "exposed",
                    DiseaseState::Infectious(_) => "infectious",
                    DiseaseState::Recovered => "recovered",
                };
                Person {
                    id: p.id,
                    px: p.position.x,
                    py: p.position.y,
                    ds: disease_state.to_string(),
                }
            })
            .collect::<Vec<_>>();

        let state = State {
            tick: self.world.tick,
            people,
        };

        JsValue::from_serde(&state).unwrap()
    }

    pub fn get_background_viral_particles(&self) -> Vec<f32> {
        let background_viral_particles =
            match self.world.config.disease_parameters.spread_parameters {
                DiseaseSpreadParameters::InfectionRadius(_) => vec![],
                DiseaseSpreadParameters::BackgroundViralParticle(_) => {
                    // TODO: downsample this.
                    self.world
                        .disease_spreader
                        .get_background_viral_levels()
                        .clone()
                }
            };

        let mut flat_particles = vec![];
        for row in background_viral_particles {
            flat_particles.extend(row);
        }

        flat_particles
    }

    pub fn get_background_viral_particles2(&mut self) -> *const f32 {
        self.background_viral_particles.clear();
        match self.world.config.disease_parameters.spread_parameters {
            DiseaseSpreadParameters::InfectionRadius(_) => {}
            DiseaseSpreadParameters::BackgroundViralParticle(_) => {
                for row in self.world.disease_spreader.get_background_viral_levels() {
                    self.background_viral_particles.extend(row);
                }
            }
        };

        &self.background_viral_particles[0]
    }

    pub fn get_households(&self) -> JsValue {
        let mut boxes = vec![];

        if let Some(map) = &self.world.map {
            // TODO: decouple this from the map: the js client currently assumes that if there is
            // a map, then we can safely call this method.
            let dual_shopper_households = self.world.person_behavior.get_dual_shopper_households();
            boxes.extend(
                map.households
                    .iter()
                    .zip_eq(dual_shopper_households.iter())
                    .map(|(h, dual_shopper)| Household {
                        bounds: BoundingBox::from_geo(&h.bounds),
                        dual_shopper: *dual_shopper,
                    }),
            );
        }

        JsValue::from_serde(&boxes).unwrap()
    }

    pub fn get_roads(&self) -> JsValue {
        let mut boxes = vec![];

        if let Some(map) = &self.world.map {
            boxes.extend(map.roads.iter().map(|r| BoundingBox::from_geo(&r.bounds)));
        }

        JsValue::from_serde(&boxes).unwrap()
    }

    pub fn get_stores(&self) -> JsValue {
        let mut boxes = vec![];

        if let Some(map) = &self.world.map {
            boxes.extend(map.stores.iter().map(|s| BoundingBox::from_geo(&s.bounds)));
        }

        JsValue::from_serde(&boxes).unwrap()
    }
}
