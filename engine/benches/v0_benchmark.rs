extern crate engine;

use criterion::{criterion_group, criterion_main, Criterion};
use engine::v0::config::{
    BackgroundViralParticleParams, DiseaseParameters, DiseaseSpreadParameters, WorldConfig,
};
use engine::v0::wasm_view::WorldView;

fn run_infection_radius_spread() {
    let world_config = WorldConfig {
        disease_parameters: DiseaseParameters {
            infectious_period_ticks: 10 * 60,
            spread_parameters: DiseaseSpreadParameters::InfectionRadius(5.0),
        },
        size: (600, 400),
        num_people: 200,
        num_initially_infected: 1,
    };

    let mut world = WorldView::new(world_config);

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
                    inhale_radius: 9.0,
                    infection_risk_per_particle: 0.0002,
                },
            ),
        },
        size: (600, 400),
        num_people: 200,
        num_initially_infected: 1,
    };

    let mut world = WorldView::new(world_config);

    for _ in 0..1800 {
        world.step();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("world steps");
    group.sample_size(20);

    group.bench_function("radius_spread", |b| b.iter(run_infection_radius_spread));

    group.bench_function("viral_particle_spread", |b| {
        b.iter(run_viral_particle_spread)
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
