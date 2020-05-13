extern crate cpuprofiler;
extern crate engine;

use cpuprofiler::PROFILER;
use criterion::{criterion_group, criterion_main, Criterion};
use engine::v0::config::{
    BackgroundViralParticleParams, BehaviorParameters, DiseaseParameters, DiseaseSpreadParameters,
    WorldConfig,
};
use engine::v0::maps;
use engine::v0::wasm_view::WorldView;

fn run_infection_radius_spread() {
    let world_config = WorldConfig {
        disease_parameters: DiseaseParameters {
            infectious_period_ticks: 10 * 60,
            spread_parameters: DiseaseSpreadParameters::InfectionRadius(5.0),
        },
        behavior_parameters: BehaviorParameters::BrownianMotion,
        size: (600, 400),
        num_people: 200,
        num_initially_infected: 1,
    };

    let rng = Box::new(rand::thread_rng());
    let mut world = WorldView::new(world_config, None, rng);

    for _ in 0..1800 {
        world.step();
    }
}

fn run_viral_particle_spread() {
    let world_config = WorldConfig {
        disease_parameters: DiseaseParameters {
            infectious_period_ticks: 10 * 60,
            spread_parameters: DiseaseSpreadParameters::BackgroundViralParticle(
                BackgroundViralParticleParams {
                    exhale_radius: 9.0,
                    exhale_amount: 1.0,
                    decay_rate: 0.5,
                    infection_risk_per_particle: 0.0002,
                },
            ),
        },
        behavior_parameters: BehaviorParameters::BrownianMotion,
        size: (600, 400),
        num_people: 200,
        num_initially_infected: 1,
    };

    let rng = Box::new(rand::thread_rng());
    let mut world = WorldView::new(world_config, None, rng);

    for _ in 0..1800 {
        world.step();
    }
}

fn run_viral_particle_spread_shopping() {
    let world_config = WorldConfig {
        disease_parameters: DiseaseParameters {
            infectious_period_ticks: 10 * 60,
            spread_parameters: DiseaseSpreadParameters::BackgroundViralParticle(
                BackgroundViralParticleParams {
                    exhale_radius: 9.0,
                    exhale_amount: 0.2,
                    decay_rate: 0.05,
                    infection_risk_per_particle: 0.00002,
                },
            ),
        },
        behavior_parameters: BehaviorParameters::Shopper {},
        size: (600, 400),
        num_people: 54,
        num_initially_infected: 1,
    };

    let map = maps::Map::load_from_ascii_str(10, maps::simple_groceries::MAP_ASCII_STR).unwrap();
    let rng = Box::new(rand::thread_rng());
    let mut world = WorldView::new(world_config, Some(map), rng);

    for _ in 0..1800 {
        world.step();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("world_steps");
    group.sample_size(10);

    PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();

    group.bench_function("radius_spread", |b| b.iter(run_infection_radius_spread));
    group.bench_function("viral_particle_spread", |b| {
        b.iter(run_viral_particle_spread)
    });
    group.bench_function("viral_particle_spread_shopping", |b| {
        b.iter(run_viral_particle_spread_shopping)
    });

    PROFILER.lock().unwrap().stop().unwrap();

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
