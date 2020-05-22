extern crate clap;
use clap::{App, Arg};
use engine::v0::config::WorldConfig;
use engine::v0::types::Mask;
use engine::v0::wasm_view::{DiseaseState, State, WorldView};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::thread;

#[derive(Serialize, Debug)]
struct EndingPersonState {
    disease_state: DiseaseState,
    dual_shopper_household: bool,
    mask: Mask,
}

#[derive(Serialize, Debug)]
struct EndingState {
    tick: usize,
    num_susceptible: usize,
    num_recovered: usize,
    ending_people_state: Vec<EndingPersonState>,
}

mod generate_config;

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

        let ending_people_state = state
            .people
            .iter()
            .map(|p| {
                let hs = &state.households[p.household];

                EndingPersonState {
                    disease_state: p.disease_state,
                    dual_shopper_household: hs.dual_shopper,
                    mask: p.mask,
                }
            })
            .collect();

        EndingState {
            tick: state.tick,
            num_susceptible,
            num_recovered,
            ending_people_state,
        }
    }
}

#[derive(Serialize, Debug)]
struct RunRecord {
    config: WorldConfig,
    // TODO: should we save the random seed?
    ending_state: EndingState,
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

fn concurrent_run_to_completion(
    mut generator: Box<dyn generate_config::ConfigGenerator>,
    mut output_file: File,
) {
    let (tx_config, rx_config) = crossbeam_channel::bounded::<WorldConfig>(0);
    let (tx_record, rx_record) = crossbeam_channel::bounded::<RunRecord>(0);

    for _ in 0..8 {
        let rx_config_clone = rx_config.clone();
        let tx_record_clone = tx_record.clone();

        thread::spawn(move || {
            while let Ok(config) = rx_config_clone.recv() {
                let rng = Box::new(rand::thread_rng());
                let mut world_view = WorldView::new(config.clone(), rng).unwrap();

                run_to_completion(&mut world_view);
                let ending_state = EndingState::from_state(&world_view.get_state());
                let record = RunRecord {
                    config,
                    ending_state,
                };

                tx_record_clone.send(record).unwrap();
            }
        });
    }

    thread::spawn(move || {
        while let Ok(record) = rx_record.recv() {
            let serialized_record = serde_json::to_string(&record)
                .unwrap_or_else(|_| panic!("failed to serialize {:?}", record));

            output_file
                .write_all(serialized_record.as_bytes())
                .expect("failed to write to file");
            output_file
                .write_all(&['\n' as u8])
                .expect("failed to write to file");

            output_file.sync_data().expect("failed to sync file");
        }
    });

    loop {
        let config = generator.gen();
        tx_config.send(config).unwrap();
    }
}

fn main() {
    let mut config_generator_by_name = {
        let mut builder: HashMap<&str, Box<dyn generate_config::ConfigGenerator>> = HashMap::new();

        builder.insert(
            "infection_radius",
            Box::new(generate_config::InfectionRadius::default()),
        );
        builder.insert(
            "viral_particle",
            Box::new(generate_config::ViralParticle::default()),
        );
        builder.insert(
            "shopping_solo",
            Box::new(generate_config::ViralParticleShoppingSolo::default()),
        );
        builder.insert(
            "shopping_mask_regular",
            Box::new(generate_config::ViralParticleShoppingMaskRegular::default()),
        );
        builder.insert(
            "shopping_mask_n95",
            Box::new(generate_config::ViralParticleShoppingMaskN95::default()),
        );
        builder.insert(
            "shopping_mask_single_n95",
            Box::new(generate_config::ViralParticleShoppingMaskSingleN95::default()),
        );

        builder
    };

    let matches = App::new("Parameter Calibration")
        .arg(
            Arg::with_name("config_generator")
                .long("config_generator")
                .takes_value(true)
                .possible_values(&config_generator_by_name.keys().cloned().collect::<Vec<_>>()),
        )
        .arg(Arg::with_name("output_file").takes_value(true))
        .get_matches();

    let config_generator_name = matches.value_of("config_generator").unwrap();
    let config_generator = config_generator_by_name
        .remove_entry(config_generator_name)
        .unwrap()
        .1;

    let output_filename = if let Some(filename) = matches.value_of("output_file") {
        filename.to_string()
    } else {
        format!("sim_data/{}_out.txt", config_generator_name)
    };

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&output_filename)
        .expect("failed to open file");

    concurrent_run_to_completion(config_generator, file);
}
