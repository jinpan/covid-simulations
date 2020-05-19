/*
import * as wasm from "wasm-engine";
import * as THREE from 'three';
import THREE_default_font_json from 'three/examples/fonts/helvetiker_bold.typeface.json';
*/
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
          "infection_risk_per_particle": 0.00013,
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
    "num_people": 108,
    "num_initially_infected": 2,
    "map_params": {
      "name": "simple_groceries",
      "scale": 10,
      "num_people_per_household": 2,
    },
  },
  "show_dual_shopper": false,
  "show_household_supplies": {
    "max_supplies": 900,
  },
  "initial_seed": 10914,
  "chart_update_period_ticks": 30,
});

// Draw the infection rate vs percent dual shopper plot
(function(){
  let xs = [];
  for (let x = 0; x <= 100; x += 2) {
    xs.push(x);
  }
  const q1 = [
    1.8, 2.7, 2.7, 2.7, 2.7, 2.7, 3.4, 2.5, 2.7, 2.7, 3.7, 3.7, 4.4,
    2.7, 5.5, 3.7, 13.8, 6.4, 6.4, 5.5, 3.7, 5.5, 7.8, 31.9, 14.3,
    16.6, 5.5, 23.1, 48.6, 13.4, 54.6, 54.8, 56.9, 58.3, 55.7, 55.7,
    56.4, 63.1, 68.5, 57.4, 65.7, 69.6, 73.1, 74.0, 76.3, 73.1, 75.2,
    80.5, 80.5, 82.64, 82.64];
  const q2 = [
    4.6, 4.1, 5.5, 4.6, 7.4, 5.5, 6.0, 4.1, 6.4, 5.5, 11.5, 8.3,
    17.1, 6.4, 23.1, 25.0, 35.1, 31.4, 36.1, 28.7, 8.3, 30.5, 44.4,
    50.0, 50.9, 48.1, 53.7, 57.4, 57.4, 60.1, 63.8, 64.8, 63.4, 63.4,
    65.7, 65.7, 69.4, 69.9, 71.7, 71.7, 69.9, 76.3, 76.3, 80.5, 79.1,
    79.6, 81.4, 84.2, 83.3, 86.1, 89.3];
  const q3 = [
    8.3, 6.4, 12.9, 21.3, 12.9, 22.2, 17.5, 12.2, 16.6, 23.8, 21.0,
    24.0, 34.9, 18.0, 38.8, 41.4, 41.6, 43.5, 46.3, 47.2, 45.3, 50.0,
    53.7, 57.4, 58.3, 59.2, 61.1, 61.5, 65.7, 65.2, 66.2, 71.3, 71.0,
    71.3, 71.3, 75.0, 73.8, 74.7, 77.7, 76.6, 80.3, 80.3, 80.5, 83.3,
    82.8, 84.0, 86.1, 86.8, 87.9, 88.8, 91.6];

  let best_fit = []
  for (let x = 0; x <= 100; x += 2) {
    const y = -3.09 + 0.96 * x;
    best_fit.push(y.toFixed(1));
  }

  let data = [
    xs,
    q1, q3, q2,
    best_fit,
  ];

  const opts = {
    width: 700,
    height: 400,
    title: "Infection Rate vs Percentage of 2x-Shopper Households",
    scales: { x: { time: false }, y: { range: [0, 100] }, },
    series: [
      { label: "%", show: false },
      {
        label: "25th pct",
        fill: "rgba(0, 0, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "75th pct",
        fill: "rgba(0, 0, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "Median",
        points: {show: false},
        value: (_, v) => v + "%",
      },
      {
        label: "Best Fit",
        dash: [1, 5],
        points: {show: false},
        value: (_, v) => v + "%",
      },
    ],
    axes: [
      {
        label: "Percentage of 2x-shopper households",
        values: (_, vals) => vals.map(v => v + "%")
      },
      {
        label: "Infection rate",
        values: (_, vals) => vals.map(v => v + "%")
      },
    ]
  };

  uplot.default(
    opts, data,
    document.getElementById("infection_rate_vs_pct_dual_shopper"));
})();

// Draw the infection rate by household type vs pct dual shopper plot
(function(){
  let xs = [];
  for (let x = 0; x <= 100; x += 2) {
    xs.push(x);
  }
  const single_q1 = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.1, 1.3,
    0.0, 2.7, 0.0, 1.4, 1.4, 3.0, 3.1, 3.6, 3.4, 5.2, 1.8, 18.7,
    12.1, 12.5, 22.9, 26.0, 27.2, 30.0, 30.0, 30.5, 30.5, 32.3,
    33.3, 36.6, 34.6, 36.3, 35.0, 35.0, 33.3, 35.7, 35.7, 33.3,
    37.5, 37.5, 33.3, 25.0, NaN
  ];
  const single_q2 = [
    1.9, 1.9, 2.0, 2.0, 3.1, 2.1, 3.1, 5.5, 4.5, 5.6, 7.1, 3.6, 12.2,
    9.8, 8.1, 15.2, 14.8, 17.6, 12.4, 23.0, 22.3, 24.6, 24.1,
    30.1, 28.1, 34.6, 32.0, 33.6, 36.6, 38.3, 37.5, 40.4, 40.0,
    41.1, 41.6, 43.3, 43.7, 43.3, 45.8, 45.8, 45.4, 45.0, 44.4,
    50.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0, NaN
  ];
  const single_q3 = [
    6.7, 6.8, 7.0, 8.1, 9.3, 10.4, 9.7, 14.1, 18.8, 17.0, 22.0, 15.1,
    25.0, 25.6, 27.6, 30.0, 31.2, 30.8, 30.6, 34.8, 35.8, 35.8,
    37.1, 39.6, 39.1, 42.5, 42.0, 43.1, 43.4, 47.1, 45.4, 47.5,
    47.3, 47.3, 50.0, 50.0, 50.0, 53.3, 53.8, 54.1, 54.5, 55.0,
    56.2, 57.1, 57.1, 58.3, 60.0, 62.5, 66.6, 75.0, NaN
  ];

  const dual_q1 = [
    NaN, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    3.1, 0.0, 0.0, 0.0, 2.5, 7.1, 9.5, 6.5, 12.6, 4.0, 28.2, 18.5,
    23.2, 48.7, 55.1, 56.6, 60.9, 65.1, 64.7, 61.4, 70.2, 69.4,
    71.0, 73.7, 73.7, 74.3, 76.7, 75.5, 78.8, 78.8, 80.4, 79.7,
    80.2, 81.6, 80.0, 81.73
  ];
  const dual_q2 = [
    NaN, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.3, 6.2, 11.1, 15.0, 9.0, 20.4, 17.2,
    18.9, 33.3, 31.2, 32.3, 31.0, 47.3, 50.0, 51.1, 56.6, 57.5,
    56.3, 64.4, 63.3, 65.5, 68.4, 71.6, 75.0, 73.4, 75.7, 75.7,
    76.4, 77.7, 79.1, 78.9, 80.7, 81.2, 82.9, 84.5, 83.3, 84.0,
    84.4, 85.8, 84.0, 85.4, 85.7, 85.0, 86.54
  ];
  const dual_q3 = [
    NaN, 0.0, 0.0, 33.3, 25.0, 20.0, 20.0, 41.6, 35.7, 33.3, 44.4, 35.0,
    51.0, 53.8, 60.1, 53.5, 62.9, 61.7, 63.8, 67.5, 72.5, 71.4,
    70.4, 74.7, 75.7, 76.9, 76.9, 77.3, 79.3, 81.6, 80.6, 81.2,
    83.3, 83.8, 83.8, 84.2, 86.1, 85.5, 86.2, 86.2, 87.8, 88.1,
    88.3, 88.6, 89.3, 89.3, 89.3, 88.5, 89.8, 89.0, 90.38
  ];

  let data = [
    xs,
    single_q1, single_q3, single_q2,
    dual_q1, dual_q3, dual_q2,
  ];

  const opts = {
    width: 700,
    height: 400,
    title: "Infection by Household Type vs % 2x-Shopper Households",
    scales: { x: { time: false }, y: { range: [0, 100] }, },
    series: [
      { label: "%" },
      {
        label: "25%",
        fill: "rgba(0, 255, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "75%",
        fill: "rgba(0, 255, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "50%",
        stroke: "rgba(0, 255, 0, 0.8)",
        points: {show: false},
        value: (_, v) => v + "%",
      },
      {
        label: "25%",
        fill: "rgba(255, 0, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "75%",
        fill: "rgba(255, 0, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "50%",
        stroke: "rgba(255, 0, 0, .8)",
        points: {show: false},
        value: (_, v) => v + "%",
      },
    ],
    axes: [
      {
        label: "Percentage of 2x-shopper households",
        values: (_, vals) => vals.map(v => v + "%")
      },
      {
        label: "Infection Rate",
        values: (_, vals) => vals.map(v => v + "%")
      },
    ]
  };

  uplot.default(
    opts, data,
    document.getElementById("infection_rate_by_household_type_vs_pct_dual_shopper"));
})();

