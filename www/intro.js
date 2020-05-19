import {Simulation, width, height} from './simulation';

const configs = (function() {
  let config_builder = {};

  let add_config = (name, data) => {
    data["name"] = name;
    config_builder[name] = data;
  };

  add_config("radius_brownian0", {
    "engine_config": {
      "disease_parameters": {
        "exposed_period_ticks": 0 * 60,
        "infectious_period_ticks": 345,
        "spread_parameters": {
          "infection_radius": 3.2,
        },
      },
      "behavior_parameters": "brownian_motion",
      "bounding_box": {
        "bottom": 0,
        "left": 0,
        "top": height,
        "right": width,
      },
      "num_people": 200,
      "num_initially_infected": 3,
    },
    "map_name": "",
    "initial_seed": 10914,
    "chart_update_period_ticks": 4,
  });
  add_config("particle_brownian0", {
    "engine_config": {
      "disease_parameters": {
        "exposed_period_ticks": 115,
        "infectious_period_ticks": 345,
        "spread_parameters": {
          "background_viral_particle": {
            "exhale_radius": 9,
            "decay_rate": 0.05,
            "infection_risk_per_particle": 0.0019,
          },
        },
      },
      "behavior_parameters": "brownian_motion",
      "bounding_box": {
        "bottom": 0,
        "left": 0,
        "top": height,
        "right": width,
      },
      "num_people": 200,
      "num_initially_infected": 3,
    },
    "map_name": "",
    "initial_seed": 10914,
    "chart_update_period_ticks": 6,
  });

  return config_builder;
})();

const simulations = [
  new Simulation(configs["radius_brownian0"]),
  new Simulation(configs["particle_brownian0"]),
];
