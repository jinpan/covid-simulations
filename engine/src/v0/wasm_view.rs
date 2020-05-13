// Provides wasm bindings for the v0 engine.
use wasm_bindgen::prelude::*;

use crate::v0::config::{DiseaseSpreadParameters, WorldConfig};
use crate::v0::core::{DiseaseState, World};
use crate::v0::geometry;
use crate::v0::maps;
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
pub struct Person {
    pub id: usize,
    pub px: f32,
    pub py: f32,
    pub ds: String,
}

#[derive(Serialize)]
pub struct State {
    pub people: Vec<Person>,
}

#[wasm_bindgen]
pub struct WorldView {
    world: World,
}

impl WorldView {
    pub fn new(config: WorldConfig, map: Option<maps::Map>, rng: Box<dyn RngCore>) -> Self {
        let world = World::new(rng, config, map);

        WorldView { world }
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

        let state = State { people };

        JsValue::from_serde(&state).unwrap()
    }

    pub fn get_background_viral_particles(&self) -> JsValue {
        let background_viral_particles =
            match self.world.config.disease_parameters.spread_parameters {
                DiseaseSpreadParameters::InfectionRadius(_) => vec![],
                DiseaseSpreadParameters::BackgroundViralParticle(_) => {
                    // TODO: downsample this.
                    self.world.disease_spreader.get_background_viral_levels(1)
                }
            };

        JsValue::from_serde(&background_viral_particles).unwrap()
    }

    pub fn get_background_viral_particles2(&self) -> Vec<f32> {
        let background_viral_particles =
            match self.world.config.disease_parameters.spread_parameters {
                DiseaseSpreadParameters::InfectionRadius(_) => vec![],
                DiseaseSpreadParameters::BackgroundViralParticle(_) => {
                    // TODO: downsample this.
                    self.world.disease_spreader.get_background_viral_levels(1)
                }
            };

        let mut flat_particles = Vec::new();
        for row in background_viral_particles {
            flat_particles.extend(row);
        }

        flat_particles
    }

    pub fn get_bounding_boxes(&self) -> JsValue {
        let mut boxes = vec![];

        if let Some(map) = &self.world.map {
            boxes.extend(
                map.households
                    .iter()
                    .map(|h| BoundingBox::from_geo(&h.bounds)),
            );
            boxes.extend(map.stores.iter().map(|s| BoundingBox::from_geo(&s.bounds)));
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
}
