// Contains configuration files for the v0 engine.

use crate::v0::geometry::BoundingBox;
use serde::{Deserialize, Serialize};

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
    pub fraction_dual_shopper_households: f32,
    pub shopping_period_ticks: usize,
    pub supplies_bought_per_trip: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BehaviorParameters {
    #[serde(rename = "brownian_motion")]
    BrownianMotion,

    #[serde(rename = "shopper")]
    Shopper(ShopperParams),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorldConfig {
    pub disease_parameters: DiseaseParameters,
    pub behavior_parameters: BehaviorParameters,
    pub bounding_box: BoundingBox,
    pub num_people: usize,
    pub num_initially_infected: usize,
}
