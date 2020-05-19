import {Simulation, width, height} from './simulation';

new Simulation({
  "name": "particle_shopper0",
  "engine_config": {
    "disease_parameters": {
      "exposed_period_ticks": 15 * 60,
      "infectious_period_ticks": 45 * 60,
      "spread_parameters": {
        "background_viral_particle": {
          "exhale_radius": 9,
          "decay_rate": 0.055,
          "infection_risk_per_particle": 0.0003,
        },
      },
    },
    "behavior_parameters": {
      "shopper": {
        "shopping_period_ticks": 5 * 60,
        "init_supply_low_range": 0,
        "init_supply_high_range": 2 * 15 * 60,
        "supplies_bought_per_trip": 15 * 60,
        "fraction_dual_shopper_households": 0,
        "fraction_bulk_shopper_households": 0.5,
        "bulk_shopper_time_multiplier": 2,
        "bulk_shopper_supplies_multiplier": 2,
      },
    },
    "bounding_box": {
      "bottom": 0,
      "left": 0,
      "top": height,
      "right": width,
    },
    "num_people": 54,
    "num_initially_infected": 2,
    "map_params": {
      "name": "simple_groceries",
      "scale": 10,
      "num_people_per_household": 1,
    },
  },
  "show_dual_shopper": false,
  "show_household_supplies": {
    "max_supplies": 2300,
  },
  "initial_seed": 10914,
  "chart_update_period_ticks": 30,
})
