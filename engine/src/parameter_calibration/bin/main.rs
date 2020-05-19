extern crate clap;
use clap::{App, Arg};
use engine::v0::config::{
    BackgroundViralParticleParams, BehaviorParameters, DiseaseParameters, DiseaseSpreadParameters,
    ShopperParams, WorldConfig,
};
use engine::v0::geometry::BoundingBox;
use engine::v0::wasm_view::{DiseaseState, State, WorldView};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;

#[derive(Serialize, Debug)]
struct EndingState {
    tick: usize,
    num_susceptible: usize,
    num_recovered: usize,
}

impl EndingState {
    fn from_state(state: &State) -> Self {
        let mut num_susceptible = 0;
        let mut num_recovered = 0;
        for person in state.people.iter() {
            match person.disease_state {
                DiseaseState::Susceptible => num_susceptible += 1,
                DiseaseState::Recovered => num_recovered += 1,
                DiseaseState::Exposed | DiseaseState::Infectious => panic!(
                    "There should not be any {:?} people at the end",
                    person.disease_state
                ),
            }
        }

        EndingState {
            tick: state.tick,
            num_susceptible,
            num_recovered,
        }
    }
}

#[derive(Serialize, Debug)]
struct RunRecord {
    config: WorldConfig,
    // TODO: should we save the random seed?
    ending_state: EndingState,
    // TODO: add human behavior-specific state
}

fn run_to_completion(world: &mut WorldView) -> usize {
    loop {
        let tick = world.step();

        if !world
            .get_state()
            .people
            .iter()
            .any(|p| match p.disease_state {
                DiseaseState::Exposed | DiseaseState::Infectious => true,
                DiseaseState::Susceptible | DiseaseState::Recovered => false,
            })
        {
            return tick;
        }
    }
}

struct RunRecorder<F>
where
    F: Fn() -> WorldConfig,
{
    generate_config_fn: F,
    output_file: File,
}

impl<F> RunRecorder<F>
where
    F: Fn() -> WorldConfig,
{
    fn add_record(&mut self) {
        let config = (self.generate_config_fn)();
        let rng = Box::new(rand::thread_rng());
        let mut world_view = WorldView::new(config.clone(), None, rng);

        run_to_completion(&mut world_view);
        let ending_state = EndingState::from_state(&world_view.get_state());

        let record = RunRecord {
            config,
            ending_state,
        };
        let serialized_record = serde_json::to_string(&record)
            .unwrap_or_else(|_| panic!("failed to serialize {:?}", record));

        self.output_file
            .write_all(serialized_record.as_bytes())
            .expect("failed to write to file");

        self.output_file.sync_data().expect("failed to sync file");
    }
}

fn generate_infection_radius_spread_config() -> WorldConfig {
    WorldConfig {
        disease_parameters: DiseaseParameters {
            exposed_period_ticks: 0,
            infectious_period_ticks: 345,
            spread_parameters: DiseaseSpreadParameters::InfectionRadius(3.2),
        },
        behavior_parameters: BehaviorParameters::BrownianMotion,
        bounding_box: BoundingBox {
            bottom: 0,
            left: 0,
            top: 400,
            right: 600,
        },
        num_people: 200,
        num_initially_infected: 3,
    }
}

fn generate_viral_particle_spread() -> WorldConfig {
    WorldConfig {
        disease_parameters: DiseaseParameters {
            exposed_period_ticks: 115,
            infectious_period_ticks: 345,
            spread_parameters: DiseaseSpreadParameters::BackgroundViralParticle(
                BackgroundViralParticleParams {
                    exhale_radius: 9.0,
                    decay_rate: 0.05,
                    infection_risk_per_particle: 0.001_9,
                },
            ),
        },
        behavior_parameters: BehaviorParameters::BrownianMotion,
        bounding_box: BoundingBox {
            bottom: 0,
            left: 0,
            top: 400,
            right: 600,
        },
        num_people: 200,
        num_initially_infected: 3,
    }
}

fn generate_viral_particle_spread_shopping() -> WorldConfig {
    // TODO: pass this upstream.
    let fraction_dual_shopper_households = 0.5;

    WorldConfig {
        disease_parameters: DiseaseParameters {
            exposed_period_ticks: 15 * 60,
            infectious_period_ticks: 45 * 60,
            spread_parameters: DiseaseSpreadParameters::BackgroundViralParticle(
                BackgroundViralParticleParams {
                    exhale_radius: 9.0,
                    decay_rate: 0.055,
                    infection_risk_per_particle: 0.000_13,
                },
            ),
        },
        behavior_parameters: BehaviorParameters::Shopper(ShopperParams {
            shopping_period_ticks: 10 * 60,
            init_supply_low_range: 150.0,
            init_supply_high_range: 450.0,
            supplies_bought_per_trip: 1800.0,
            fraction_dual_shopper_households,
            fraction_bulk_shopper_households: 0.0,
            bulk_shopper_time_multiplier: 0.0,
            bulk_shopper_supplies_multiplier: 0.0,
        }),
        bounding_box: BoundingBox {
            bottom: 0,
            left: 0,
            top: 400,
            right: 600,
        },
        num_people: 108,
        num_initially_infected: 2,
    }
}

fn main() {
    let config_name_to_fn = {
        let mut builder: HashMap<&str, &dyn Fn() -> WorldConfig> = HashMap::new();

        builder.insert("infection_radius", &generate_infection_radius_spread_config);
        builder.insert("viral_particle", &generate_viral_particle_spread);
        builder.insert(
            "viral_particle_shopper",
            &generate_viral_particle_spread_shopping,
        );

        builder
    };

    let matches = App::new("Parameter Calibration")
        .arg(
            Arg::with_name("generate_config_fn")
                .takes_value(true)
                .possible_values(&config_name_to_fn.keys().cloned().collect::<Vec<_>>()),
        )
        .arg(
            Arg::with_name("output_file")
                .takes_value(true)
                .default_value("simulation_data.txt"),
        )
        .get_matches();

    let generate_config_fn = config_name_to_fn
        .get(matches.value_of("generate_config_fn").unwrap())
        .unwrap();

    let output_filename = matches.value_of("output_file").unwrap();

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_filename)
        .expect("failed to open file");
    let mut run_recorder = RunRecorder {
        generate_config_fn,
        output_file: file,
    };

    run_recorder.add_record();
}
