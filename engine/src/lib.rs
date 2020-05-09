extern crate approx;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

pub mod v0;

use crate::v0::config::WorldConfig;
use crate::v0::wasm_view::WorldView;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn create_world(config: &JsValue) -> WorldView {
    log(&format!("Received config: {:?}", config));

    let world_config: WorldConfig = config.into_serde().expect("failed to parse");

    log(&format!("Parsed config: {:?}", world_config));

    WorldView::new(world_config)
}
