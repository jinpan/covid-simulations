// Provides wasm bindings for the v0 engine.
use wasm_bindgen::prelude::*;

use crate::v0::config::{DiseaseSpreadParameters, WorldConfig};
use crate::v0::core;
use crate::v0::geometry::BoundingBox;
use anyhow::Result;
use rand::RngCore;
use serde::Serialize;

#[derive(Serialize, Debug, Copy, Clone)]
pub enum DiseaseState {
    #[serde(rename = "susceptible")]
    Susceptible,

    #[serde(rename = "exposed")]
    Exposed,

    #[serde(rename = "infectious")]
    Infectious,

    #[serde(rename = "recovered")]
    Recovered,
}

impl DiseaseState {
    fn from_core(ds: &core::DiseaseState) -> Self {
        match ds {
            core::DiseaseState::Susceptible => DiseaseState::Susceptible,
            core::DiseaseState::Exposed(_) => DiseaseState::Exposed,
            core::DiseaseState::Infectious(_) => DiseaseState::Infectious,
            core::DiseaseState::Recovered => DiseaseState::Recovered,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct HouseholdState {
    pub bounds: BoundingBox,
    pub dual_shopper: bool,
    pub bulk_shopper: bool,
    pub supply_levels: f32,
}

impl HouseholdState {
    fn from_bounds(bounds: BoundingBox) -> Self {
        HouseholdState {
            bounds,
            dual_shopper: false,
            bulk_shopper: false,
            supply_levels: 0.0,
        }
    }
}

#[derive(Serialize)]
pub struct Person {
    pub id: usize,

    #[serde(rename = "px")]
    pub position_x: f32,

    #[serde(rename = "py")]
    pub position_y: f32,

    #[serde(rename = "ds")]
    pub disease_state: DiseaseState,

    #[serde(skip)]
    pub household: usize,
}

#[derive(Serialize)]
pub struct State {
    pub tick: usize,
    pub people: Vec<Person>,
    pub households: Vec<HouseholdState>,
}

#[wasm_bindgen]
pub struct WorldView {
    world: core::World,

    background_viral_particles: Vec<f32>,
}

impl WorldView {
    pub fn new(config: WorldConfig, rng: Box<dyn RngCore>) -> Result<Self> {
        let world = core::World::new(rng, config)?;

        let background_viral_particles = Vec::with_capacity(world.config.bounding_box.size());

        Ok(WorldView {
            world,
            background_viral_particles,
        })
    }

    pub fn get_state(&self) -> State {
        let people = self
            .world
            .people
            .iter()
            .map(|p| Person {
                id: p.id,
                position_x: p.position.x,
                position_y: p.position.y,
                disease_state: DiseaseState::from_core(&p.disease_state),
                household: p.household_idx,
            })
            .collect::<Vec<_>>();

        let mut households = vec![];
        if let Some(map) = &self.world.map {
            households.extend(map.households.iter().enumerate().map(|(idx, h)| {
                let mut hs = HouseholdState::from_bounds(h.bounds);
                self.world
                    .person_behavior
                    .update_household_state(idx, &mut hs);

                hs
            }));
        }

        State {
            tick: self.world.tick,
            people,
            households,
        }
    }
}

#[wasm_bindgen]
impl WorldView {
    pub fn step(&mut self) -> usize {
        self.world.step();
        self.world.tick
    }

    pub fn to_json(&self) -> JsValue {
        let state = self.get_state();

        JsValue::from_serde(&state).unwrap()
    }

    pub fn get_background_viral_particles(&mut self) -> js_sys::Float32Array {
        self.background_viral_particles.clear();
        match self.world.config.disease_parameters.spread_parameters {
            DiseaseSpreadParameters::InfectionRadius(_) => {}
            DiseaseSpreadParameters::BackgroundViralParticle(_) => {
                for row in self.world.disease_spreader.get_background_viral_levels() {
                    self.background_viral_particles.extend(row);
                }
            }
        };

        unsafe { js_sys::Float32Array::view(self.background_viral_particles.as_slice()) }
    }

    pub fn get_households(&self) -> JsValue {
        let mut boxes = vec![];

        if let Some(map) = &self.world.map {
            boxes.extend(map.households.iter().enumerate().map(|(idx, h)| {
                let mut hs = HouseholdState::from_bounds(h.bounds);
                self.world
                    .person_behavior
                    .update_household_state(idx, &mut hs);

                hs
            }));
        }

        JsValue::from_serde(&boxes).unwrap()
    }

    pub fn get_roads(&self) -> JsValue {
        let mut boxes = vec![];

        if let Some(map) = &self.world.map {
            boxes.extend(map.roads.iter().map(|r| r.bounds));
        }

        JsValue::from_serde(&boxes).unwrap()
    }

    pub fn get_stores(&self) -> JsValue {
        let mut boxes = vec![];

        if let Some(map) = &self.world.map {
            boxes.extend(map.stores.iter().map(|s| s.bounds));
        }

        JsValue::from_serde(&boxes).unwrap()
    }
}
