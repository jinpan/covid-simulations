// Provides wasm bindings for the v0 engine.
use wasm_bindgen::prelude::*;

use crate::v0::core::{DiseaseState, World, WorldConfig};

#[derive(Serialize)]
pub struct Person {
    pub id: String,
    pub position_x: f32,
    pub position_y: f32,
    pub color: u32,
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
    pub(crate) fn new(config: WorldConfig) -> Self {
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
                let color = match p.disease_state {
                    DiseaseState::Susceptible => 0xff0000,
                    DiseaseState::Infectious(_) => 0x00ff00,
                    DiseaseState::Recovered => 0x0000ff,
                };
                Person {
                    id: format!("{}", idx),
                    position_x: p.position_and_direction.position.x,
                    position_y: p.position_and_direction.position.y,
                    color,
                }
            })
            .collect::<Vec<_>>();
        let state = State { people };

        JsValue::from_serde(&state).unwrap()
    }
}
