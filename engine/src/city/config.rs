#[derive(Debug)]
pub enum DistributionParams {
    Fixed(f64),
    Normal(/*mean*/ f64, /*stddev*/ f64),
    LogNormal(/*mean*/ f64, /*stddev*/ f64),
}

#[derive(Debug)]
pub struct DiseaseParams {
    pub latent_period_d: DistributionParams,
    pub serial_interval_d: DistributionParams, // may be negative
    pub symptomatic_period_d: DistributionParams,
    pub asymptomatic_period_d: DistributionParams,

    pub transmission_rate_for_symptomatic: f64,
    pub transmission_rate_for_asymptomatic: f64,

    pub probability_asymptomatic: DistributionParams,

    pub self_quarantine_after_symptomatic_h: usize,
    pub self_quarantine_after_infectious_h: usize,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum BuildingType {
    Residence,
    School,
    Office,
    Restaurant,
    Store,
    Other,
    // Hospitals are excluded because they may act a residence, which does not currently fit in with
    // the modeling.
}

#[derive(Debug)]
pub struct LatLon {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub struct BuildingConfig {
    pub building_type: BuildingType,
    pub maximum_occupancy: usize,
    pub location: LatLon,
    pub square_footage: u32,
}

pub struct CityConfig {
    // The number of people is determined by the number of residences, max occupancy
    pub buildings: Vec<BuildingConfig>,

    pub disease_params: DiseaseParams,
    pub initial_latent_percent: f32,
}
