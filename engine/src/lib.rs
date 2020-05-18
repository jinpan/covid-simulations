extern crate approx;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

pub mod v0;

use crate::v0::config::WorldConfig;
use crate::v0::maps;
use crate::v0::wasm_view::WorldView;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn create_world(config: &JsValue, map_name: &str, maybe_seed: Option<u32>) -> WorldView {
    let world_config: WorldConfig = config.into_serde().expect("failed to parse");

    let map = match map_name {
        "" => None,
        _ => Some(maps::loader::load(map_name, 10).unwrap()),
    };

    let rng: Box<dyn RngCore> = if let Some(seed_val) = maybe_seed {
        Box::new(ChaCha8Rng::seed_from_u64(seed_val as u64))
    } else {
        Box::new(rand::thread_rng())
    };
    WorldView::new(world_config, map, rng)
}
