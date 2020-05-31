use clap::{App, Arg};
use engine::city::config::{
    BuildingConfig, BuildingType, CityConfig, DiseaseParams, DistributionParams, LatLon,
};
use engine::city::types::CityState;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::fs::OpenOptions;
use std::io::Write;

fn make_config() -> CityConfig {
    // 100,000 people, split into 25,000 residences
    let mut buildings = Vec::new();
    for _ in 0..25_000 {
        buildings.push(BuildingConfig {
            building_type: BuildingType::Residence,
            maximum_occupancy: 4,
            location: LatLon {
                latitude: 0.0,
                longitude: 0.0,
            },
            square_footage: 1000,
        });
    }

    // 20,000 students, split into 1,000 schools
    for _ in 0..1_000 {
        buildings.push(BuildingConfig {
            building_type: BuildingType::School,
            maximum_occupancy: 20,
            location: LatLon {
                latitude: 0.0,
                longitude: 0.0,
            },
            square_footage: 1000,
        });
    }

    // 50,000 workers, split into 2,500 offices
    for _ in 0..500 {
        buildings.push(BuildingConfig {
            building_type: BuildingType::Office,
            maximum_occupancy: 100,
            location: LatLon {
                latitude: 0.0,
                longitude: 0.0,
            },
            square_footage: 2000,
        });
    }

    CityConfig {
        buildings,
        disease_params: DiseaseParams {
            latent_period_d: DistributionParams::LogNormal(1.57, 0.65),
            serial_interval_d: DistributionParams::Normal(3.96, 4.75),
            symptomatic_period_d: DistributionParams::LogNormal(2.95491, 0.033),
            asymptomatic_period_d: DistributionParams::LogNormal(2.95491, 0.033),
            transmission_rate_for_symptomatic: 0.01,
            transmission_rate_for_asymptomatic: 0.005,
            probability_asymptomatic: DistributionParams::Fixed(0.28),
            self_quarantine_after_symptomatic_h: 48, // WAG: 24h to get tested, 24h for results
            // self_quarantine_after_infectious_h: 480000,
            self_quarantine_after_infectious_h: 48,
        },
        initial_latent_percent: 0.01, // 100k people, 1k initially infected
    }
}

fn main() {
    let matches = App::new("CitySim")
        .arg(Arg::with_name("output_file").takes_value(true))
        .get_matches();

    let output_filename = matches.value_of("output_file").unwrap();

    let rng: Box<dyn RngCore> = Box::new(ChaCha8Rng::seed_from_u64(10914));
    let config = make_config();
    let mut city_state = CityState::from_config(rng, config);
    // println!("hello {:#?}", city_state);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_filename)
        .expect("failed to open file");

    for _ in 0..4380 {
        city_state.step();

        let snapshot = city_state.to_snapshot();
        let serialized_record = serde_json::to_string(&snapshot)
            .unwrap_or_else(|_| panic!("failed to serialize {:?}", snapshot));

        file.write_all(serialized_record.as_bytes())
            .expect("failed to write to file");
        file.write_all(&['\n' as u8])
            .expect("failed to write to file");
    }
    file.sync_data().expect("failed to sync file");
}
