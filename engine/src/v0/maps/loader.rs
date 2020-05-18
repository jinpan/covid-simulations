use crate::v0::maps::Map;

use crate::v0::maps;
use anyhow::{anyhow, Result};

pub fn load(map_name: &str, scale_factor: u8) -> Result<Map> {
    let map_ascii_str = match map_name {
        "simple_groceries" => maps::simple_groceries::MAP_ASCII_STR,
        _ => {
            return Err(anyhow!("unknown map name {}", map_name));
        }
    };

    maps::Map::load_from_ascii_str(scale_factor, map_ascii_str)
}
