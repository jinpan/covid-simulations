// Contains the core implementation of the v0 engine.

use crate::v0::config::*;
use crate::v0::disease_spread::{
    BackgroundViralParticleDiseaseSpreader, DiseaseSpreader, InfectionRadiusDiseaseSpreader,
};
use crate::v0::geometry::Position;
use crate::v0::maps;
use crate::v0::person_behavior::{BrownianMotionBehavior, PersonBehavior, ShopperBehavior};
use crate::v0::types::Mask;
use crate::v0::utils::{random_bool_vec, random_vec};
use anyhow::Result;
use rand::RngCore;

#[derive(PartialEq, Debug)]
pub(crate) enum DiseaseState {
    Susceptible,
    Exposed(usize),    // Tick of when the person was exposed
    Infectious(usize), // Tick of when the person was infected
    Recovered,
}

pub(crate) struct Person {
    pub(crate) id: usize,
    pub(crate) disease_state: DiseaseState,
    pub(crate) position: Position,

    // Index into the map's household for the person if the map exists.
    // Otherwise, 0.
    pub(crate) household_idx: usize,
    pub(crate) head_of_household: bool,
    pub(crate) mask: Mask,
}

pub(crate) struct World {
    pub(crate) config: WorldConfig,
    pub(crate) map: Option<maps::Map>,
    pub(crate) tick: usize,

    pub(crate) people: Vec<Person>,

    // TODO: refactor for static dispatch
    pub(crate) disease_spreader: Box<dyn DiseaseSpreader>,
    pub(crate) person_behavior: Box<dyn PersonBehavior>,

    rng: Box<dyn RngCore>,
}

impl World {
    pub(crate) fn new(mut rng: Box<dyn RngCore>, config: WorldConfig) -> Result<Self> {
        // Load the map
        let maybe_map = if let Some(map_params) = &config.map_params {
            Some(maps::loader::load(map_params)?)
        } else {
            None
        };

        assert!(config.num_people >= config.num_initially_infected);

        let pct_initially_infected =
            config.num_initially_infected as f32 / config.num_people as f32;
        let infected_people = random_bool_vec(config.num_people, pct_initially_infected, &mut rng);

        let masks = random_vec(
            config.num_people,
            Mask::Regular,
            config.misc_parameters.fraction_mask,
            Mask::N95,
            config.misc_parameters.fraction_n95_mask,
            Mask::None,
            &mut rng,
        );

        let mut current_household_idx = 0;
        let mut people_in_current_household = 0;
        let people = (0..config.num_people)
            .map(|i| {
                let disease_state = if infected_people[i] {
                    DiseaseState::Infectious(0)
                } else {
                    DiseaseState::Susceptible
                };

                let bb = if let Some(map) = &maybe_map {
                    let household = &map.households[current_household_idx];
                    if people_in_current_household == household.num_people {
                        current_household_idx += 1;
                        people_in_current_household = 0;
                    }
                    people_in_current_household += 1;

                    map.households[current_household_idx].bounds
                } else {
                    config.bounding_box
                };
                let position = bb.random_point(&mut rng);

                Person {
                    id: i,
                    disease_state,
                    position,
                    household_idx: current_household_idx,
                    head_of_household: people_in_current_household == 1,
                    mask: masks[i],
                }
            })
            .collect::<Vec<_>>();

        let disease_spreader: Box<dyn DiseaseSpreader> =
            match config.disease_parameters.spread_parameters {
                DiseaseSpreadParameters::InfectionRadius(r) => {
                    Box::new(InfectionRadiusDiseaseSpreader::new(r))
                }

                DiseaseSpreadParameters::BackgroundViralParticle(params) => Box::new(
                    BackgroundViralParticleDiseaseSpreader::new(config.bounding_box, params),
                ),
            };

        let person_behavior: Box<dyn PersonBehavior> = match config.behavior_parameters {
            BehaviorParameters::BrownianMotion => Box::new(BrownianMotionBehavior::new(
                config.bounding_box,
                config.num_people,
                &mut rng,
            )),
            BehaviorParameters::Shopper(params) => Box::new(ShopperBehavior::new(
                config.bounding_box,
                params,
                &people,
                maybe_map
                    .as_ref()
                    .expect("must have map for shopper behavior"),
                &mut rng,
            )),
        };

        Ok(World {
            config,
            map: maybe_map,
            tick: 0,
            people,
            disease_spreader,
            person_behavior,
            rng,
        })
    }

    pub fn step(&mut self) {
        self.tick += 1;
        let tick = self.tick;

        // Step 1: advance all the people
        self.person_behavior
            .update_positions(&mut self.people, &mut self.map, &mut self.rng);

        // Step 2: Update disease state according to the spread model.
        self.disease_spreader
            .spread(tick, &mut self.rng, &mut self.people);

        // Step 3: Update time-based disease states:
        //   * Advance exposed states to infectious
        //   * Advance infectious states to recovered
        let disease_parameters = &self.config.disease_parameters;
        self.people.iter_mut().for_each(|p| match p.disease_state {
            DiseaseState::Exposed(start_tick) => {
                if tick - start_tick >= disease_parameters.exposed_period_ticks {
                    p.disease_state = DiseaseState::Infectious(tick);
                }
            }
            DiseaseState::Infectious(start_tick) => {
                if tick - start_tick >= disease_parameters.infectious_period_ticks {
                    p.disease_state = DiseaseState::Recovered;
                }
            }
            DiseaseState::Susceptible | DiseaseState::Recovered => {}
        });
    }
}
