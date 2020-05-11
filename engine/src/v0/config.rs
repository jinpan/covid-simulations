// Contains configuration files for the v0 engine.

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct BackgroundViralParticleParams {
    // How far an infected person spreads background particles
    // TODO: this should vary as a function of PPE.
    pub exhale_radius: f32,

    // TODO: add parameter for exhale density function: the amount of particles exhaled should
    // not be uniform across the circle.

    // The quantity of particles an infected person emits per unit square per tick
    pub exhale_amount: f32,

    // What percentage of particles deactivate each tick.
    // 0 means that no particles are ever deactivated, 1 means that all particles are instantly
    // deactivated.
    pub decay_rate: f32,

    // How far a person inhales particles from.
    // TODO: this should vary as a function of PPE.
    pub inhale_radius: f32,

    // TODO: add a parameter for inhale density function: the amount of particles inhaled should
    // not be uniform across the circle.

    // Probability per inhaled viral particle per tick that a susceptible person becomes
    // infected.
    pub infection_risk_per_particle: f32,
}

#[derive(Deserialize, Debug)]
pub enum DiseaseSpreadParameters {
    #[serde(rename = "infection_radius")]
    InfectionRadius(f32),

    #[serde(rename = "background_viral_particle")]
    BackgroundViralParticle(BackgroundViralParticleParams),
}

#[derive(Deserialize, Debug)]
pub struct DiseaseParameters {
    pub infectious_period_ticks: usize,

    pub spread_parameters: DiseaseSpreadParameters,
}

#[derive(Deserialize, Debug)]
pub enum BehaviorParameters {
    #[serde(rename = "brownian_motion")]
    BrownianMotion,

    #[serde(rename = "shopper")]
    Shopper,
}

#[derive(Deserialize, Debug)]
pub struct WorldConfig {
    pub disease_parameters: DiseaseParameters,
    pub behavior_parameters: BehaviorParameters,
    pub size: (u16, u16),
    pub num_people: usize,
    pub num_initially_infected: usize,
}
