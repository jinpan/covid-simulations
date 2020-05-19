use crate::v0::maps::Map;

use crate::v0::config::MapParams;
use crate::v0::maps;
use anyhow::{anyhow, Result};

pub fn load(params: &MapParams) -> Result<Map> {
    let map_ascii_str = match params.name.as_str() {
        "simple_groceries" => maps::simple_groceries::MAP_ASCII_STR,
        _ => {
            return Err(anyhow!("unknown map name {}", params.name));
        }
    };

    maps::Map::load_from_ascii_str(map_ascii_str, params.scale, params.num_people_per_household)
}
