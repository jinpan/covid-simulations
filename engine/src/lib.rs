extern crate approx;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

mod v0;

use crate::v0::core::WorldConfig;
use crate::v0::wasm_view::WorldView;
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn create_world(config: &JsValue) -> WorldView {
    let world_config: WorldConfig = config.into_serde().expect("failed to parse");

    log(&format!("Received config: {:?}", world_config));

    let world = WorldView::new(world_config);

    world
}

/*
pub fn foo() -> v0::World {
    let mut rng = Box::new(rand::thread_rng());
    log(&format!("rand: {}", rng.gen_range(0, 10)));

    let mut world = v0::World::new(rng, (400, 300), 200, 1);
    for _ in 1..1000 {
        world.step();
    }

    world
}
     */
