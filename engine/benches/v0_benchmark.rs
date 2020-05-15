extern crate cpuprofiler;
extern crate engine;

use cpuprofiler::PROFILER;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use engine::v0::config::{
    BackgroundViralParticleParams, BehaviorParameters, DiseaseParameters, DiseaseSpreadParameters,
    ShopperParams, WorldConfig,
};
use engine::v0::maps;
use engine::v0::wasm_view::{DiseaseState, State, WorldView};

#[derive(Default)]
struct EndingDiseaseStatePercentages {
    susceptible: f32,
    exposed: f32,
    infectious: f32,
    recovered: f32,
}

impl EndingDiseaseStatePercentages {
    fn from_state(state: &State) -> Self {
        let mut record = EndingDiseaseStatePercentages::default();

        let num_people = state.people.len() as f32;
        for person in state.people.iter() {
            match person.disease_state {
                DiseaseState::Susceptible => record.susceptible += 100.0 / num_people,
                DiseaseState::Exposed => record.exposed += 100.0 / num_people,
                DiseaseState::Infectious => record.infectious += 100.0 / num_people,
                DiseaseState::Recovered => record.recovered += 100.0 / num_people,
            }
        }

        record
    }
}

#[derive(Default)]
struct EndingDiseaseStateCountsRecorder {
    records: Vec<EndingDiseaseStatePercentages>,
}

impl EndingDiseaseStateCountsRecorder {
    fn add_record(&mut self, world_view: &WorldView) {
        let record = EndingDiseaseStatePercentages::from_state(&world_view.get_state());

        self.records.push(record);

        if self.records.len() % 100 == 0 {
            self.print_summary();
        }
    }

    fn print_summary(&self) {
        println!("{} records", self.records.len());

        // returns the 5/25/50/75/95 percentiles of the data
        macro_rules! print_percentiles {
            ($field:ident) => {{
                let mut values = self.records.iter().map(|r| r.$field).collect::<Vec<_>>();
                values.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let p025 = (self.records.len() as f32 * 0.025) as usize;
                let p25 = (self.records.len() as f32 * 0.25) as usize;
                let p50 = (self.records.len() as f32 * 0.50) as usize;
                let p75 = (self.records.len() as f32 * 0.75) as usize;
                let p975 = (self.records.len() as f32 * 0.975) as usize;

                println!(
                    "Percentiles of {:12}: {:2}/{:2}/{:2}/{:2}/{:2}",
                    stringify!($field),
                    values[p025] as i32,
                    values[p25] as i32,
                    values[p50] as i32,
                    values[p75] as i32,
                    values[p975] as i32,
                );
            }};
        }

        print_percentiles!(susceptible);
        print_percentiles!(exposed);
        print_percentiles!(infectious);
        print_percentiles!(recovered);
    }
}

fn run_infection_radius_spread(recorder: &mut EndingDiseaseStateCountsRecorder) {
    let world_config = WorldConfig {
        disease_parameters: DiseaseParameters {
            exposed_period_ticks: 0,
            infectious_period_ticks: 345,
            spread_parameters: DiseaseSpreadParameters::InfectionRadius(3.2),
        },
        behavior_parameters: BehaviorParameters::BrownianMotion,
        size: (600, 400),
        num_people: 200,
        num_initially_infected: 3,
    };

    let rng = Box::new(rand::thread_rng());
    let mut world = WorldView::new(world_config, None, rng);

    for _ in 0..3000 {
        world.step();
    }

    recorder.add_record(&world);
}

fn run_viral_particle_spread(recorder: &mut EndingDiseaseStateCountsRecorder) {
    let world_config = WorldConfig {
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
        size: (600, 400),
        num_people: 200,
        num_initially_infected: 3,
    };

    let rng = Box::new(rand::thread_rng());
    let mut world = WorldView::new(world_config, None, rng);

    for _ in 0..5000 {
        world.step();
    }

    recorder.add_record(&world);
}

fn run_viral_particle_spread_shopping(recorder: &mut EndingDiseaseStateCountsRecorder) {
    let world_config = WorldConfig {
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
            fraction_dual_shopper_households: 0.25,
            shopping_period_ticks: 10 * 60,
            supplies_bought_per_trip: 1800.0,
        }),
        size: (600, 400),
        num_people: 108,
        num_initially_infected: 2,
    };

    let map = maps::Map::load_from_ascii_str(10, maps::simple_groceries::MAP_ASCII_STR).unwrap();
    let rng = Box::new(rand::thread_rng());
    let mut world = WorldView::new(world_config, Some(map), rng);

    for idx in 0..40000 {
        world.step();

        if idx > 1000 {
            let record = EndingDiseaseStatePercentages::from_state(&world.get_state());
            if record.exposed == 0.0 && record.infectious == 0.0 {
                break;
            }
        }
    }

    recorder.add_record(&world);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("world_steps");
    group.sample_size(20);

    PROFILER
        .lock()
        .unwrap()
        .start("/tmp/my-prof.profile")
        .unwrap();

    let mut output_recorder = EndingDiseaseStateCountsRecorder::default();
    /*
    group.bench_function("radius_spread", |b| {
        b.iter(|| run_infection_radius_spread(black_box(&mut output_recorder)))
    });
    group.bench_function("viral_particle_spread", |b| {
        b.iter(|| run_viral_particle_spread(black_box(&mut output_recorder)))
    });
    */
    group.bench_function("viral_particle_spread_shopping", |b| {
        b.iter(|| run_viral_particle_spread_shopping(black_box(&mut output_recorder)))
    });

    PROFILER.lock().unwrap().stop().unwrap();

    output_recorder.print_summary();

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
