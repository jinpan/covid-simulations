// Contains configuration files for the v0 engine.

use crate::v0::geometry::BoundingBox;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MapParams {
    pub name: String,
    pub scale: u8,
    pub num_people_per_household: u8,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct BackgroundViralParticleParams {
    // How far an infected person spreads background particles
    // TODO: this should vary as a function of PPE.
    pub exhale_radius: f32,

    // TODO: add parameter for exhale density function: the amount of particles exhaled should
    // not be uniform across the circle.

    // What percentage of particles deactivate each tick.
    // 0 means that no particles are ever deactivated, 1 means that all particles are instantly
    // deactivated.
    pub decay_rate: f32,

    // Probability per inhaled viral particle per tick that a susceptible person becomes
    // infected.
    pub infection_risk_per_particle: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DiseaseSpreadParameters {
    #[serde(rename = "infection_radius")]
    InfectionRadius(f32),

    #[serde(rename = "background_viral_particle")]
    BackgroundViralParticle(BackgroundViralParticleParams),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DiseaseParameters {
    pub exposed_period_ticks: usize,
    pub infectious_period_ticks: usize,

    pub spread_parameters: DiseaseSpreadParameters,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct ShopperParams {
    pub shopping_period_ticks: usize,

    pub init_supply_low_range: f32,
    pub init_supply_high_range: f32,
    pub supplies_bought_per_trip: f32,

    pub fraction_dual_shopper_households: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BehaviorParameters {
    #[serde(rename = "brownian_motion")]
    BrownianMotion,

    #[serde(rename = "shopper")]
    Shopper(ShopperParams),
}

// As the simulation grows, these parameters should be grouped together and moved out.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MiscParams {
    // Fraction of people who have a mask
    pub fraction_mask: f32,

    // Fraction of people who have a n95 mask
    pub fraction_n95_mask: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorldConfig {
    pub disease_parameters: DiseaseParameters,
    pub behavior_parameters: BehaviorParameters,
    pub bounding_box: BoundingBox,
    pub num_people: usize,
    pub num_initially_infected: usize,
    pub misc_parameters: MiscParams,
    pub map_params: Option<MapParams>,
}
