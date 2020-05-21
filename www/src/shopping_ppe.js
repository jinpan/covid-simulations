import 'uplot/dist/uPlot.min.css';
import * as uplot from 'uplot';
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
          "infection_risk_per_particle": 0.0004,
          "fraction_mask": 0.5,
          "fraction_n95_mask": 0.0,
        },
      },
    },
    "behavior_parameters": {
      "shopper": {
        "shopping_period_ticks": 10 * 60,
        "init_supply_low_range": 150,
        "init_supply_high_range": 450,
        "supplies_bought_per_trip": 30 * 60,
        "fraction_dual_shopper_households": 0.5,
        "fraction_bulk_shopper_households": 0,
        "bulk_shopper_time_multiplier": 0,
        "bulk_shopper_supplies_multiplier": 0,
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
    "misc_parameters": {
      "fraction_mask": 0.5,
      "fraction_n95_mask": 0.0,
    },
  },
  "show_dual_shopper": false,
  "show_household_supplies": {
    "max_supplies": 900,
  },
  "initial_seed": 10914,
  "chart_update_period_ticks": 30,
});
