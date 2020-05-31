use crate::city::types::{CityState, DiseaseStatus};
use rand::Rng;
use std::collections::HashMap;

impl CityState {
    // Advance time by 1 hour.
    pub fn step(&mut self) {
        self.hour += 1;

        // Step 1: update disease states
        let disease_params = &self.disease_params;
        let hour = self.hour;
        let mut rng = &mut self.rng;
        self.people.iter_mut().for_each(|p| {
            let next_time = p.disease_state.next_status_time;

            match p.disease_state.status {
                DiseaseStatus::Latent => {
                    if hour >= next_time {
                        // Roll the RNG to determine the next state
                        let p_asymptotic = disease_params.probability_asymptomatic.sample(&mut rng);
                        if rng.gen_bool(p_asymptotic) {
                            p.disease_state.status = DiseaseStatus::InfASym;
                            p.disease_state.next_status_time = hour
                                + (24.0 * disease_params.asymptomatic_period_d.sample(rng))
                                    as usize;
                        } else {
                            if disease_params.serial_interval_d.mean() >= 0.0 {
                                p.disease_state.status = DiseaseStatus::InfSym;
                                p.disease_state.next_status_time = hour
                                    + (24.0 * disease_params.symptomatic_period_d.sample(rng))
                                        as usize;
                            } else {
                                p.disease_state.status = DiseaseStatus::InfPreSym;
                                p.disease_state.next_status_time = hour
                                    + (24.0 * disease_params.serial_interval_d.sample(rng).abs())
                                        as usize;
                            }
                        }
                        p.disease_state.status_start_time = hour;
                    }
                }
                DiseaseStatus::InfPreSym => {
                    if hour >= next_time {
                        p.disease_state.status = DiseaseStatus::InfSym;
                        p.disease_state.next_status_time = hour
                            + (24.0 * disease_params.symptomatic_period_d.sample(rng)) as usize;
                        p.disease_state.status_start_time = hour;
                    }
                }
                DiseaseStatus::InfASym | DiseaseStatus::InfSym => {
                    if hour >= next_time {
                        p.disease_state.status = DiseaseStatus::Recovered;
                        p.disease_state.next_status_time = 0;
                        p.disease_state.status_start_time = hour;
                    }
                }
                DiseaseStatus::Susceptible | DiseaseStatus::Recovered => {}
            }
        });

        // Step 2: infectious people infect those around them
        // Group people by building
        // Note that due to the way we group, the people within the building are sorted, so we have
        // a stable notion of neighbors, which mimics the "synthetic social network" in comokit.
        // TODO: only sort within schools / offices.
        // We can imagine that the people are lined up in a space filling Hilbert curve, so it is
        // sufficient for infectious people to infect their left/right N neighbors, instead of
        // needing to do a more complicated 2D search.
        let mut people_id_by_building_id = HashMap::<usize, Vec<usize>>::new();
        self.people.iter().for_each(|p| {
            let entry = people_id_by_building_id.entry(p.current_building_id);
            entry.or_default().push(p.id);
        });
        for people_ids in people_id_by_building_id.values() {
            for i in 0..people_ids.len() {
                let p = &self.people[people_ids[i]];

                let p_transmit = match p.disease_state.status {
                    DiseaseStatus::Susceptible
                    | DiseaseStatus::Latent
                    | DiseaseStatus::Recovered => continue,
                    DiseaseStatus::InfASym | DiseaseStatus::InfPreSym => {
                        disease_params.transmission_rate_for_asymptomatic
                    }
                    DiseaseStatus::InfSym => disease_params.transmission_rate_for_symptomatic,
                };

                let neighbor_idxs = [
                    (i + people_ids.len() - 1) % people_ids.len(),
                    (i + 1) % people_ids.len(),
                ];

                let mut num_people_infected = 0;
                for &neighbor_idx in neighbor_idxs.iter() {
                    let p2 = &mut self.people[people_ids[neighbor_idx]];
                    if let DiseaseStatus::Susceptible = p2.disease_state.status {
                        // Roll dice on infection
                        if rng.gen_bool(p_transmit) {
                            p2.disease_state.status = DiseaseStatus::Latent;
                            p2.disease_state.next_status_time =
                                hour + (24.0 * disease_params.latent_period_d.sample(rng)) as usize;

                            num_people_infected += 1;
                        }
                    }
                }

                let p = &mut self.people[people_ids[i]];
                p.num_people_infected += num_people_infected;
            }
        }

        // Step 3: people move to their next location
        let hour_mod24 = self.hour % 24;
        self.people.iter_mut().for_each(|p| {
            // Check if the person is self quarantining
            let mut self_quarantine = false;

            if let DiseaseStatus::InfSym = p.disease_state.status {
                if hour - p.disease_state.status_start_time
                    > disease_params.self_quarantine_after_symptomatic_h
                {
                    self_quarantine = true;
                }
            }

            if p.disease_state.status.is_infectious()
                && hour - p.disease_state.status_start_time
                    > disease_params.self_quarantine_after_infectious_h
            {
                self_quarantine = true;
            }

            if self_quarantine {
                p.current_building_id = p.home_building_id;
            } else {
                p.current_building_id = p.schedule[hour_mod24];
            }
        });
    }
}
