// City-scale simulations.
// Cities have on the order of 100K people.
// People are modeled as entities that teleport between buildings.
// Infections are transferred within buildings, according to some transmission probability
// This transmission probability will be tuned so R is inline with real world measurements.

pub mod config;
pub mod core;
pub mod types;
