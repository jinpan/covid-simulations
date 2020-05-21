extern crate cpuprofiler;
extern crate engine;
// extern crate flame;

use cpuprofiler::PROFILER;
use criterion::{criterion_group, criterion_main, Criterion};
use engine::v0::config::{
    BackgroundViralParticleParams, BehaviorParameters, DiseaseParameters, DiseaseSpreadParameters,
    MapParams, MiscParams, ShopperParams, WorldConfig,
};
use engine::v0::geometry::BoundingBox;
use engine::v0::wasm_view::WorldView;
// use std::fs::File;

fn run_infection_radius_spread() {
    let world_config = WorldConfig {
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
        map_params: None,
        misc_parameters: MiscParams {
            fraction_mask: 0.0,
            fraction_n95_mask: 0.0,
        },
    };

    let rng = Box::new(rand::thread_rng());
    let mut world = WorldView::new(world_config, rng).unwrap();

    for _ in 0..3600 {
        world.step();
    }
}

fn run_viral_particle_spread() {
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
        bounding_box: BoundingBox {
            bottom: 0,
            left: 0,
            top: 400,
            right: 600,
        },
        num_people: 200,
        num_initially_infected: 3,
        map_params: None,
        misc_parameters: MiscParams {
            fraction_mask: 0.0,
            fraction_n95_mask: 0.0,
        },
    };

    let rng = Box::new(rand::thread_rng());
    let mut world = WorldView::new(world_config, rng).unwrap();

    for _ in 0..3600 {
        world.step();
    }
}

fn run_viral_particle_spread_shopping() {
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
            shopping_period_ticks: 10 * 60,
            init_supply_low_range: 150.0,
            init_supply_high_range: 450.0,
            supplies_bought_per_trip: 1800.0,
            fraction_dual_shopper_households: 0.5,
        }),
        bounding_box: BoundingBox {
            bottom: 0,
            left: 0,
            top: 400,
            right: 600,
        },
        num_people: 108,
        num_initially_infected: 2,
        map_params: Some(MapParams {
            name: "simple_groceries".to_string(),
            scale: 10,
            num_people_per_household: 2,
        }),
        misc_parameters: MiscParams {
            fraction_mask: 0.0,
            fraction_n95_mask: 0.0,
        },
    };

    let rng = Box::new(rand::thread_rng());
    let mut world = WorldView::new(world_config, rng).unwrap();

    for _ in 0..3600 {
        world.step();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("world_steps");
    group.sample_size(10);

    PROFILER
        .lock()
        .unwrap()
        .start("/tmp/my-prof.profile")
        .unwrap();

    group.bench_function("radius_spread", |b| b.iter(run_infection_radius_spread));
    group.bench_function("viral_particle_spread", |b| {
        b.iter(run_viral_particle_spread)
    });
    group.bench_function("viral_particle_spread_shopping", |b| {
        b.iter(run_viral_particle_spread_shopping)
    });

    PROFILER.lock().unwrap().stop().unwrap();

    run_viral_particle_spread();
    // flame::dump_html(&mut File::create("/tmp/flame-graph.html").unwrap()).unwrap();

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
