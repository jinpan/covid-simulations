// Provides wasm bindings for the v0 engine.
use wasm_bindgen::prelude::*;

use crate::v0::config::{DiseaseSpreadParameters, WorldConfig};
use crate::v0::core::{DiseaseState, World};

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
    pub background_viral_particles: Vec<Vec<f32>>,
}

#[wasm_bindgen]
pub struct WorldView {
    world: World,
}

impl WorldView {
    pub fn new(config: WorldConfig) -> Self {
        let rng = Box::new(rand::thread_rng());

        let world = World::new(rng, config);

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
            .enumerate()
            .map(|(idx, p)| {
                let disease_state = match p.disease_state {
                    DiseaseState::Susceptible => "susceptible",
                    DiseaseState::Infectious(_) => "infectious",
                    DiseaseState::Recovered => "recovered",
                };
                Person {
                    id: idx,
                    px: p.position_and_direction.position.x,
                    py: p.position_and_direction.position.y,
                    ds: disease_state.to_string(),
                }
            })
            .collect::<Vec<_>>();

        let background_viral_particles =
            match self.world.config.disease_parameters.spread_parameters {
                DiseaseSpreadParameters::InfectionRadius(_) => vec![],
                DiseaseSpreadParameters::BackgroundViralParticle(_) => {
                    // TODO: downsample this.
                    self.world.background_viral_particles.clone()
                }
            };

        let state = State {
            people,
            background_viral_particles,
        };

        JsValue::from_serde(&state).unwrap()
    }
}
