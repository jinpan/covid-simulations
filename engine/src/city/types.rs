use crate::city::config::{
    BuildingConfig, BuildingType, CityConfig, DiseaseParams, DistributionParams,
};
use derivative::Derivative;
use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use rand::{Rng, RngCore};
use serde::Serialize;

// These values are based on COMOKIT's model
#[derive(Debug)]
pub enum DiseaseStatus {
    Susceptible,
    Latent,    // Not infectious, analogous to exposed state in SEIR
    InfPreSym, // Infectious pre-symptomatic
    InfASym,   // Infectious a-symptomatic
    InfSym,    // Infectious symptomatic
    Recovered,
}

impl DiseaseStatus {
    pub(crate) fn is_infectious(&self) -> bool {
        match self {
            DiseaseStatus::Susceptible => false,
            DiseaseStatus::Latent => false,
            DiseaseStatus::InfPreSym => true,
            DiseaseStatus::InfASym => true,
            DiseaseStatus::InfSym => true,
            DiseaseStatus::Recovered => false,
        }
    }
}

impl DistributionParams {
    pub(crate) fn sample(&self, rng: &mut dyn RngCore) -> f64 {
        match self {
            DistributionParams::Fixed(x) => *x,
            DistributionParams::Normal(mu, stddev) => {
                let dist = rand_distr::Normal::new(*mu, *stddev).unwrap();
                dist.sample(rng)
            }
            DistributionParams::LogNormal(mu, stddev) => {
                let dist = rand_distr::LogNormal::new(*mu, *stddev).unwrap();
                dist.sample(rng)
            }
        }
    }

    pub(crate) fn mean(&self) -> f64 {
        match self {
            DistributionParams::Fixed(x) => *x,
            DistributionParams::Normal(mu, _) => *mu,
            DistributionParams::LogNormal(_, _) => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct DiseaseState {
    pub(crate) status: DiseaseStatus,
    pub(crate) status_start_time: usize, // Hour of when this status was first changed to
    pub(crate) next_status_time: usize,  // Hour of when to transition to the next status
}

fn make_schedule(home_building_id: usize, secondary_building_id: Option<usize>) -> [usize; 24] {
    let mut schedule = [home_building_id; 24];

    if let Some(n) = secondary_building_id {
        for i in 9..17 {
            schedule[i] = n;
        }
    }

    // TODO: add more activities

    schedule
}

#[derive(Debug)]
pub struct PersonState {
    pub(crate) id: usize,
    pub(crate) home_building_id: usize,
    // This is a school/office building
    secondary_building_id: Option<usize>,

    pub(crate) current_building_id: usize,
    pub(crate) disease_state: DiseaseState,

    pub(crate) schedule: [usize; 24],

    pub(crate) num_people_infected: usize,
}

#[derive(Debug)]
pub struct Building {
    config: BuildingConfig,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct CityState {
    #[derivative(Debug = "ignore")]
    pub(crate) rng: Box<dyn RngCore>,
    pub(crate) disease_params: DiseaseParams,

    pub(crate) hour: usize,

    pub(crate) people: Vec<PersonState>,
    pub(crate) buildings: Vec<Building>,
}

#[derive(Debug, Serialize)]
pub struct CityStateSnapshot {
    hour: usize,
    num_susceptible: usize,
    num_latent: usize,
    num_infectious: usize,
    num_recovered: usize,
    r_eff: f32,
}

impl CityState {
    pub fn from_config(mut rng: Box<dyn RngCore>, config: CityConfig) -> Self {
        let mut people = Vec::new();
        config.buildings.iter().enumerate().for_each(|(i, b)| {
            if let BuildingType::Residence = b.building_type {
            } else {
                return;
            }

            for _ in 0..b.maximum_occupancy {
                let id = people.len();

                let home_building_id = i;

                let schedule = make_schedule(home_building_id, None);

                people.push(PersonState {
                    id,
                    home_building_id,
                    secondary_building_id: None,
                    current_building_id: i,
                    disease_state: DiseaseState {
                        status: DiseaseStatus::Susceptible,
                        status_start_time: 0,
                        next_status_time: 0,
                    },
                    schedule,
                    num_people_infected: 0,
                });
            }
        });

        let num_initially_infected =
            (people.len() as f32 * config.initial_latent_percent).round() as usize;
        let mut initially_infected_people = vec![false; people.len()];
        for i in 0..num_initially_infected {
            initially_infected_people[i] = true;
        }
        initially_infected_people.shuffle(&mut rng);

        let mut secondary_building_ids = Vec::new();
        config.buildings.iter().enumerate().for_each(|(i, b)| {
            match b.building_type {
                BuildingType::School | BuildingType::Office => {}
                _ => return,
            };

            for _ in 0..b.maximum_occupancy {
                secondary_building_ids.push(Some(i));
            }
        });
        assert!(secondary_building_ids.len() < people.len());
        let num_stay_at_home = people.len() - secondary_building_ids.len();
        for _ in 0..num_stay_at_home {
            secondary_building_ids.push(None);
        }
        secondary_building_ids.shuffle(&mut rng);

        for (i, p) in people.iter_mut().enumerate() {
            if initially_infected_people[i] {
                p.disease_state.status = DiseaseStatus::Latent;
                let latent_hrs =
                    (24.0 * config.disease_params.latent_period_d.sample(&mut rng)) as usize;
                p.disease_state.next_status_time = rng.gen_range(0, latent_hrs);
            }

            p.secondary_building_id = secondary_building_ids[i];
            p.schedule = make_schedule(p.home_building_id, p.secondary_building_id);
        }

        let buildings = config
            .buildings
            .into_iter()
            .map(|b| Building { config: b })
            .collect();

        CityState {
            rng,
            disease_params: config.disease_params,
            hour: 0,
            people,
            buildings,
        }
    }

    pub fn to_snapshot(&self) -> CityStateSnapshot {
        let mut num_susceptible = 0;
        let mut num_latent = 0;
        let mut num_infectious = 0;
        let mut num_recovered = 0;

        let mut num_infected_people = 0;
        let mut num_infectious_for_a_while = 0;

        self.people.iter().for_each(|p| {
            match p.disease_state.status {
                DiseaseStatus::Susceptible => num_susceptible += 1,
                DiseaseStatus::Latent => num_latent += 1,
                DiseaseStatus::InfPreSym => num_infectious += 1,
                DiseaseStatus::InfASym => num_infectious += 1,
                DiseaseStatus::InfSym => num_infectious += 1,
                DiseaseStatus::Recovered => num_recovered += 1,
            };

            // Exclude people who have been infectious for <166h from R calculations
            let N = 166;
            if p.disease_state.status.is_infectious()
                && self.hour > p.disease_state.status_start_time + N
            {
                num_infected_people += p.num_people_infected;
                num_infectious_for_a_while += 1;
            }
            if let DiseaseStatus::Recovered = p.disease_state.status {
                if self.hour < p.disease_state.status_start_time + N {
                    num_infected_people += p.num_people_infected;
                    num_infectious_for_a_while += 1;
                }
            }
        });

        let r_eff = (num_infected_people as f32) / (num_infectious_for_a_while as f32);

        CityStateSnapshot {
            hour: self.hour,
            num_susceptible,
            num_latent,
            num_infectious,
            num_recovered,
            r_eff,
        }
    }
}
