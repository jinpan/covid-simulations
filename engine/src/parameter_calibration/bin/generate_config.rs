use engine::v0::config::{
    BackgroundViralParticleParams, BehaviorParameters, DiseaseParameters, DiseaseSpreadParameters,
    MapParams, MiscParams, ShopperParams, WorldConfig,
};
use engine::v0::geometry::BoundingBox;

pub(crate) trait ConfigGenerator {
    fn gen(&mut self) -> WorldConfig;
}

#[derive(Default)]
pub(crate) struct InfectionRadius {}
impl ConfigGenerator for InfectionRadius {
    fn gen(&mut self) -> WorldConfig {
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
            map_params: None,
            misc_parameters: MiscParams {
                fraction_mask: 0.0,
                fraction_n95_mask: 0.0,
            },
        }
    }
}

#[derive(Default)]
pub(crate) struct ViralParticle {}
impl ConfigGenerator for ViralParticle {
    fn gen(&mut self) -> WorldConfig {
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
            map_params: None,
            misc_parameters: MiscParams {
                fraction_mask: 0.0,
                fraction_n95_mask: 0.0,
            },
        }
    }
}

#[derive(Default)]
pub(crate) struct ViralParticleShoppingSolo {
    fraction_dual_shopper_households: f32,
}

impl ViralParticleShoppingSolo {
    fn get_fraction_dual_shopper_households(&mut self) -> f32 {
        let val = self.fraction_dual_shopper_households;

        self.fraction_dual_shopper_households += 0.01;
        if self.fraction_dual_shopper_households > 1.0 {
            self.fraction_dual_shopper_households = 0.0;
        }

        val
    }
}

impl ConfigGenerator for ViralParticleShoppingSolo {
    fn gen(&mut self) -> WorldConfig {
        let fraction_dual_shopper_households = self.get_fraction_dual_shopper_households();

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
        }
    }
}

#[derive(Default)]
pub(crate) struct ViralParticleShoppingMaskRegular {
    fraction_mask: f32,
}

impl ViralParticleShoppingMaskRegular {
    fn get_fraction_mask(&mut self) -> f32 {
        let val = self.fraction_mask;

        self.fraction_mask += 0.01;
        if self.fraction_mask > 1.0 {
            self.fraction_mask = 0.0;
        }

        val
    }
}

impl ConfigGenerator for ViralParticleShoppingMaskRegular {
    fn gen(&mut self) -> WorldConfig {
        let fraction_mask = self.get_fraction_mask();

        WorldConfig {
            disease_parameters: DiseaseParameters {
                exposed_period_ticks: 15 * 60,
                infectious_period_ticks: 45 * 60,
                spread_parameters: DiseaseSpreadParameters::BackgroundViralParticle(
                    BackgroundViralParticleParams {
                        exhale_radius: 9.0,
                        decay_rate: 0.055,
                        infection_risk_per_particle: 0.000_4,
                    },
                ),
            },
            behavior_parameters: BehaviorParameters::Shopper(ShopperParams {
                shopping_period_ticks: 10 * 60,
                init_supply_low_range: 150.0,
                init_supply_high_range: 450.0,
                supplies_bought_per_trip: 1800.0,
                fraction_dual_shopper_households: 0.0,
            }),
            bounding_box: BoundingBox {
                bottom: 0,
                left: 0,
                top: 400,
                right: 600,
            },
            num_people: 54,
            num_initially_infected: 2,
            map_params: Some(MapParams {
                name: "simple_groceries".to_string(),
                scale: 10,
                num_people_per_household: 1,
            }),
            misc_parameters: MiscParams {
                fraction_mask,
                fraction_n95_mask: 0.0,
            },
        }
    }
}

#[derive(Default)]
pub(crate) struct ViralParticleShoppingMaskN95 {
    fraction_n95_mask: f32,
}

impl ViralParticleShoppingMaskN95 {
    fn get_fraction_n95_mask(&mut self) -> f32 {
        let val = self.fraction_n95_mask;

        self.fraction_n95_mask += 0.01;
        if self.fraction_n95_mask > 1.0 {
            self.fraction_n95_mask = 0.0;
        }

        val
    }
}

impl ConfigGenerator for ViralParticleShoppingMaskN95 {
    fn gen(&mut self) -> WorldConfig {
        let fraction_n95_mask = self.get_fraction_n95_mask();

        WorldConfig {
            disease_parameters: DiseaseParameters {
                exposed_period_ticks: 15 * 60,
                infectious_period_ticks: 45 * 60,
                spread_parameters: DiseaseSpreadParameters::BackgroundViralParticle(
                    BackgroundViralParticleParams {
                        exhale_radius: 9.0,
                        decay_rate: 0.055,
                        infection_risk_per_particle: 0.000_4,
                    },
                ),
            },
            behavior_parameters: BehaviorParameters::Shopper(ShopperParams {
                shopping_period_ticks: 10 * 60,
                init_supply_low_range: 150.0,
                init_supply_high_range: 450.0,
                supplies_bought_per_trip: 1800.0,
                fraction_dual_shopper_households: 0.0,
            }),
            bounding_box: BoundingBox {
                bottom: 0,
                left: 0,
                top: 400,
                right: 600,
            },
            num_people: 54,
            num_initially_infected: 2,
            map_params: Some(MapParams {
                name: "simple_groceries".to_string(),
                scale: 10,
                num_people_per_household: 1,
            }),
            misc_parameters: MiscParams {
                fraction_mask: 0.0,
                fraction_n95_mask,
            },
        }
    }
}

#[derive(Default)]
pub(crate) struct ViralParticleShoppingMaskSingleN95 {
    fraction_mask: f32,
}

impl ViralParticleShoppingMaskSingleN95 {
    fn get_fraction_mask(&mut self) -> f32 {
        let val = self.fraction_mask;

        self.fraction_mask += 0.01;
        if self.fraction_mask >= 0.98 {
            self.fraction_mask = 0.0;
        }

        val
    }
}

impl ConfigGenerator for ViralParticleShoppingMaskSingleN95 {
    fn gen(&mut self) -> WorldConfig {
        let fraction_mask = self.get_fraction_mask();

        WorldConfig {
            disease_parameters: DiseaseParameters {
                exposed_period_ticks: 15 * 60,
                infectious_period_ticks: 45 * 60,
                spread_parameters: DiseaseSpreadParameters::BackgroundViralParticle(
                    BackgroundViralParticleParams {
                        exhale_radius: 9.0,
                        decay_rate: 0.055,
                        infection_risk_per_particle: 0.000_4,
                    },
                ),
            },
            behavior_parameters: BehaviorParameters::Shopper(ShopperParams {
                shopping_period_ticks: 10 * 60,
                init_supply_low_range: 150.0,
                init_supply_high_range: 450.0,
                supplies_bought_per_trip: 1800.0,
                fraction_dual_shopper_households: 0.0,
            }),
            bounding_box: BoundingBox {
                bottom: 0,
                left: 0,
                top: 400,
                right: 600,
            },
            num_people: 54,
            num_initially_infected: 2,
            map_params: Some(MapParams {
                name: "simple_groceries".to_string(),
                scale: 10,
                num_people_per_household: 1,
            }),
            misc_parameters: MiscParams {
                fraction_mask,
                fraction_n95_mask: 0.0185,
            },
        }
    }
}
