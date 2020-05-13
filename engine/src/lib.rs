extern crate approx;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

pub mod v0;

use crate::v0::config::WorldConfig;
use crate::v0::maps;
use crate::v0::wasm_view::WorldView;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn create_world(config: &JsValue, map_name: &str) -> WorldView {
    log(&format!("Received config: {:?}", config));
    let world_config: WorldConfig = config.into_serde().expect("failed to parse");
    log(&format!("Parsed config: {:?}", world_config));

    let map = match map_name {
        "" => None,
        "simple_groceries" => {
            Some(maps::Map::load_from_ascii_str(10, maps::simple_groceries::MAP_ASCII_STR).unwrap())
        }
        _ => panic!("Unknown map <{}>", map_name),
    };

    let rng = Box::new(rand::thread_rng());
    WorldView::new(world_config, map, rng)
}
