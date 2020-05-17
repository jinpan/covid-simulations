import * as wasm from "wasm-engine";
import * as THREE from 'three';
import * as uplot from 'uplot';
import 'uplot/dist/uPlot.min.css';
import THREE_default_font_json from 'three/examples/fonts/helvetiker_bold.typeface.json';

let THREE_default_font = (new THREE.FontLoader()).parse(THREE_default_font_json);

window._wasm = wasm;
window._uplot = uplot;

const width = 600;
const height = 400;

const color_map = {
  "susceptible": 0xB8F7BF,
  "exposed": 0xC7BA29,
  "infectious": 0xEB6383,
  "recovered": 0xC8C8C8,
};
const uplot_opts = {
  // width is set dynamically
  height: 50,
  scales: { x: { time: false }, y: { range: [0, 200] }, },
  axes: [ { show: false }, { show: false }, ],
  cursor: { show: false },
  legend: { show: false },
  series: [
    {},
    { stroke: "gray", fill: "rgba(200,200,200,1)", points: { show: false } },
    { stroke: "green", fill: "rgba(184,247,191,1)", points: { show: false } },
    { stroke: "red", fill: "rgba(235,99,131,1)", points: { show: false } },
    { stroke: "yellow", fill: "rgba(199,186,41,1)", points: { show: false } },
  ],
};

class Simulation {
  constructor(config) {
    this.config = Object.assign({}, config);  // Deep copy of the config
    this.world = wasm.create_world(config.engine_config, config.map_name, config.initial_seed);

    this._play = false;
    this.speed = 1;

    this.reset_uplot(this.config);
    this.reset_three(this.config, this.world);

    this.register_buttons();
  }

  reset_uplot() {
    this.uplot_el = document.getElementById(`${this.config.name}-uplot`);
    let uplot_width = this.uplot_el.clientWidth;

    let opts_copy = Object.assign({}, uplot_opts);
    opts_copy['scales']['y']['range'] = [0, this.config['engine_config']['num_people']];
    opts_copy['width'] = uplot_width;

    this.uplot_data = [
      Array.from(Array(uplot_width).keys()),
      [], // susceptible
      [], // exposed + susceptible
      [], // exposed + infectious + susceptible
      [], // exposed + recovered + infectious + susceptible
    ];
    if (this.uplot_inst === undefined) {
      this.uplot_inst = new uplot.default(
        opts_copy, this.uplot_data,
        this.uplot_el,
      );
    } else {
      // A uplot instance has been previously created.
      // Reset the data for it.
      this.uplot_inst.setData(this.uplot_data);
    }
    this.next_chart_update_tick = 0;
  }

  reset_three() {
    // The garbage collector will take care of deleting the old scene.
    this.scene = new THREE.Scene();

    if (this.config.map_name != "") {
      this.draw_map(this.world, this.scene);
    }

    this.draw_people(this.world, this.scene);
    this.draw_background(this.world, this.scene);

    this.camera = new THREE.OrthographicCamera(
        0, width, height, 0,
        0, 1000,
    );
    this.camera.position.z = 5;

    let canvas = document.getElementById(`${this.config.name}-canvas`);
    this.renderer = new THREE.WebGLRenderer({ "canvas": canvas });
    this.renderer.sortObjects = false;
    this.renderer.setClearColor (0xfafafa, 1);
    this.renderer.render(this.scene, this.camera);
  }

  draw_map(world, scene) {
    // Draw households
    let household_material = new THREE.MeshBasicMaterial();
    let household_text_material = new THREE.MeshBasicMaterial({
      "color": 0xAAAAAA, "side": THREE.DoubleSide,
      "transparent": true, "opacity": 0.4,
    });
    for (const household of world.get_households()) {
      let box = household.bounds;
      let width = box.right - box.left;
      let height = box.bot - box.top;

      let plane_geo = new THREE.PlaneGeometry(width, height, 1);
      let plane = new THREE.Mesh(plane_geo, household_material);

      plane.position.x = (box.left + box.right) / 2;
      plane.position.y = (box.bot + box.top) / 2;

      let plane_box = new THREE.BoxHelper(plane, 0x000000);
      scene.add(plane_box);

      let msg = (household.dual_shopper) ? "2x" : "1x";
      let text_geo = new THREE.TextGeometry(msg, {
        "font": THREE_default_font,
        "size": 15,
      });
      let text = new THREE.Mesh(text_geo, household_text_material);

      text.position.x = box.left + 5;
      text.position.y = box.top + 2;

      scene.add(text);
    }

    // Draw stores
    let store_material = new THREE.MeshBasicMaterial();
    for (const box of world.get_stores()) {
      let width = box.right - box.left;
      let height = box.bot - box.top;

      let plane_geo = new THREE.PlaneGeometry(width, height, 1);
      let plane = new THREE.Mesh(plane_geo, store_material);

      plane.position.x = (box.left + box.right) / 2;
      plane.position.y = (box.bot + box.top) / 2;

      let plane_box = new THREE.BoxHelper(plane, 0x000000);
      scene.add(plane_box);
    }

    // Draw roads
    let road_material = new THREE.MeshBasicMaterial({
      "color": 0x333333,
    });
    for (const box of world.get_roads()) {
      let width = box.right - box.left;
      let height = box.bot - box.top;

      let plane_geo = new THREE.PlaneGeometry(width, height, 1);
      let plane = new THREE.Mesh(plane_geo, road_material);

      plane.position.x = (box.left + box.right) / 2;
      plane.position.y = (box.bot + box.top) / 2;

      scene.add(plane);
    }
  }

  draw_people(world, scene) {
    let circle_geo = new THREE.CircleGeometry( 4, 32 );

    this.people_by_id = new Map();
    for (const person_state of world.to_json()["people"]) {
      let color = color_map[person_state["ds"]];
      let material = new THREE.MeshBasicMaterial( { color: color } );
      let person = new THREE.Mesh( circle_geo, material );

      person.position.x = person_state["px"];
      person.position.y = person_state["py"];

      this.people_by_id.set(person_state["id"], person);
      scene.add(person);
    }
  }

  draw_background(world, scene) {
    let size = width * height;
    this.background_color_data = new Uint8Array( 3 * size );

    for ( let i = 0; i < size; i ++ ) {
      let stride = i * 3;

      this.background_color_data[ stride ] = Math.floor(250);
      this.background_color_data[ stride + 1] = Math.floor(250);
      this.background_color_data[ stride + 2] = Math.floor(250);
    }
    this.texture = new THREE.DataTexture( this.background_color_data, width, height, THREE.RGBFormat );
    scene.background = this.texture;
  }

  animate_people(people) {
    for (const person_state of people) {
      let person = this.people_by_id.get(person_state["id"]);

      person.position.x = person_state["px"];
      person.position.y = person_state["py"];

      let color = color_map[person_state["ds"]];
      person.material.color.setHex(color);
    }
  }

  update_chart(people) {
    let counts = {
      "susceptible": 0,
      "exposed": 0,
      "infectious": 0,
      "recovered": 0,
    };
    for (const person of people) {
      counts[person.ds] += 1;
    }

    const a = counts["exposed"];
    const b = a + counts["infectious"];
    const c = b + counts["susceptible"];
    const d = c + counts["recovered"];

    this.uplot_data[1].push(d);
    this.uplot_data[2].push(c);
    this.uplot_data[3].push(b);
    this.uplot_data[4].push(a);

    if (this.uplot_data[1].length > this.uplot_data[0].length) {
      for (let i=1; i<=4; i++) {
        this.uplot_data[i].shift();
      }
    }

    // Check to see if we need to resize this container.
    let uplot_width = this.uplot_el.clientWidth;
    if (this.uplot_data[0].length != uplot_width) {
      this.uplot_data[0] = Array.from(Array(uplot_width).keys());

      const diff = this.uplot_data[1].length - this.uplot_data[0].length;
      if (diff > 0) {
        for (let i=1; i<=4; i++) {
          this.uplot_data[i] = this.uplot_data[i].slice(diff);
        }
      }
      this.uplot_inst.setSize({'width': uplot_width, 'height': uplot_opts['height']});
    }
    this.uplot_inst.setData(this.uplot_data);
  }

  update_background_viral_particles() {
    // let background_viral_particles = world.get_background_viral_particles();
    // Directly create a Float32Array view here from the wasm buffer to avoid allocating a copy.
    // Profiles show significantly less memory allocator pressure from this.
    const background_viral_particles = new Float32Array(
      wasm._WASM_MEMORY.buffer,
      this.world.get_background_viral_particles2(),
      width * height,
    );

    for (let idx = 0; idx < width * height; idx++) {
      const stride = idx * 3;
      const val = background_viral_particles[idx];

      // Math.min/max is really slow on safari/ios
      let red = 250 + val;
      red = (red > 255) ? 255 : red;
      let greenblue = 250 - 8*val;
      greenblue = (greenblue < 0) ? 0 : 250 - 8*val;

      this.background_color_data[stride] = red;
      this.background_color_data[stride+1] = greenblue;
      this.background_color_data[stride+2] = greenblue;
    }

    this.texture.needsUpdate = true;
  }

  animate() {
    if (!this._play) {
      return;
    }
    requestAnimationFrame( () => {this.animate();} );

    let state = null;
    for (let i = 0; i < this.speed; i++) {
      const tick = this.world.step();
      if (tick > this.next_chart_update_tick) {
        this.next_chart_update_tick += this.config["chart_update_period_ticks"];

        state = this.world.to_json();
        this.update_chart(state["people"]);
      }
    }
    if (state == null) {
      state = this.world.to_json();
    }

    this.animate_people(state["people"]);
    let spread_params = this.config["engine_config"]["disease_parameters"]["spread_parameters"];
    if ('background_viral_particle' in spread_params) {
      this.update_background_viral_particles();
    }


    this.renderer.render( this.scene, this.camera );
  }

  // Controller related utilities
  play() {
    const was_paused = !this._play;
    this._play = true;

    if (was_paused) {
      // Pause other simulations.
      for (let i = 0; i < simulations.length; i++) {
        let sim = simulations[i];
        if (sim != this) {
          sim.pause();
        }
      }

      let start_btn = document.getElementById(`${this.config.name}-start`);
      start_btn.innerText = "Pause";
      this.animate();
    }
  }
  pause() {
    this._play = false;
    let start_btn = document.getElementById(`${this.config.name}-start`);
    start_btn.innerText = "Start";
  }

  reset() {
    this.world.free();
    // Only the first world is deterministically seeded.
    // Subsequent worlds are randomly seeded.
    this.world = wasm.create_world(this.config.engine_config, this.config.map_name);

    this.reset_uplot();
    this.reset_three();
  }

  register_buttons() {
    let sim = this;
    const cfg_name = sim.config.name;

    // Start/Pause button
    let start_btn = document.getElementById(`${cfg_name}-start`);
    start_btn.addEventListener("click", function() {
      if (sim._play) {
        sim.pause();
      } else {
        sim.play();
      }
    });

    // Reset button
    let reset_btn = document.getElementById(`${cfg_name}-reset`);
    reset_btn.addEventListener("click", function() {
      sim.reset();
    });

    // Speed buttons
    for (let btn of document.getElementsByClassName(`${cfg_name}-speed`)) {
      btn.addEventListener("click", function() {
        // Update button appearances
        for (let btn2 of document.getElementsByClassName(`${cfg_name}-speed`)) {
          btn2.style["font-weight"] = "normal";
          btn2.disabled = false;
        }
        this.style["font-weight"] = "bold";
        this.disabled = true;

        sim.speed = parseInt(this.dataset.speed);
      });
    };

    for (let btn of document.getElementsByClassName(`${cfg_name}-pct-dual-shopper`)) {
      btn.addEventListener("click", function() {
        // Update button appearances
        for (let btn2 of document.getElementsByClassName(`${cfg_name}-pct-dual-shopper`)) {
          btn2.style["font-weight"] = "normal";
          btn2.disabled = false;
        }
        this.style["font-weight"] = "bold";
        this.disabled = true;

        const fraction_dual_shopper = parseInt(this.dataset.pct) / 100;
        let shopper_params = sim.config['engine_config']['behavior_parameters']['shopper'];
        shopper_params['fraction_dual_shopper_households'] = fraction_dual_shopper;

        sim.reset();
      });
    };
  }
}

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
      "size": [width, height],
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
      "size": [width, height],
      "num_people": 200,
      "num_initially_infected": 3,
    },
    "map_name": "",
    "initial_seed": 10914,
    "chart_update_period_ticks": 6,
  });
  add_config("particle_shopper0", {
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
          "fraction_dual_shopper_households": 0.5,
          "shopping_period_ticks": 10 * 60,
          "supplies_bought_per_trip": 30 * 60,
        },
      },
      "size": [width, height],
      "num_people": 108,
      "num_initially_infected": 2,
    },
    "map_name": "simple_groceries",
    "initial_seed": 10914,
    "chart_update_period_ticks": 30,
  });

  return config_builder;
})();

const simulations = (function() {
  let simulations_builder = [];

  for (const config_name in configs) {
    simulations_builder.push(new Simulation(configs[config_name]));
  }

  return simulations_builder;
})();

// Draw the infection rate vs percent dual shopper plot
(function(){
  let xs = [];
  for (let x = 0; x <= 100; x += 2) {
    xs.push(x);
  }
  const q1 = [
    1.85, 2.78, 2.78, 2.78, 2.78, 2.78, 3.47, 2.55, 2.78, 2.78, 3.7, 3.7, 4.4,
    2.78, 5.56, 3.7, 13.89, 6.48, 6.48, 5.56, 3.7, 5.56, 7.87, 31.94, 14.35,
    16.67, 5.56, 23.15, 48.61, 13.43, 54.63, 54.86, 56.94, 58.33, 55.79, 55.79,
    56.48, 63.19, 68.52, 57.41, 65.74, 69.68, 73.15, 74.07, 76.39, 73.15, 75.23,
    80.56, 80.56, 82.64, 82.64];
  const q2 = [
    4.63, 4.17, 5.56, 4.63, 7.41, 5.56, 6.02, 4.17, 6.48, 5.56, 11.57, 8.33,
    17.13, 6.48, 23.15, 25.0, 35.19, 31.48, 36.11, 28.7, 8.33, 30.56, 44.44,
    50.0, 50.93, 48.15, 53.7, 57.41, 57.41, 60.19, 63.89, 64.81, 63.43, 63.43,
    65.74, 65.74, 69.44, 69.91, 71.76, 71.76, 69.91, 76.39, 76.39, 80.56, 79.17,
    79.63, 81.48, 84.26, 83.33, 86.11, 89.35];
  const q3 = [
    8.33, 6.48, 12.96, 21.3, 12.96, 22.22, 17.59, 12.27, 16.67, 23.84, 21.06,
    24.07, 34.95, 18.06, 38.89, 41.44, 41.67, 43.52, 46.3, 47.22, 45.37, 50.0,
    53.7, 57.41, 58.33, 59.26, 61.11, 61.57, 65.74, 65.28, 66.2, 71.3, 71.06,
    71.3, 71.3, 75.0, 73.84, 74.77, 77.78, 76.62, 80.32, 80.32, 80.56, 83.33,
    82.87, 84.03, 86.11, 86.81, 87.96, 88.89, 91.67];

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
      { },
      {
        label: "25th percentile",
        fill: "rgba(0, 0, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "75th percentile",
        fill: "rgba(0, 0, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "Median",
        dash: [1, 5],
        points: {show: false},
        value: (_, v) => v + "%",
      },
      {
        label: "Linear Regression",
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
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.14, 1.31,
    0.0, 2.7, 0.0, 1.47, 1.47, 3.03, 3.15, 3.67, 3.45, 5.22, 1.85, 18.7,
    12.12, 12.5, 22.92, 26.09, 27.27, 30.0, 30.0, 30.56, 30.56, 32.35,
    33.33, 36.67, 34.62, 36.36, 35.0, 35.0, 33.33, 35.71, 35.71, 33.33,
    37.5, 37.5, 33.33, 25.0, NaN
  ];
  const single_q2 = [
    1.92, 1.96, 2.0, 2.04, 3.12, 2.13, 3.19, 5.56, 4.55, 5.68, 7.14, 3.66, 12.2,
    9.87, 8.11, 15.28, 14.86, 17.65, 12.49, 23.08, 22.3, 24.6, 24.17,
    30.18, 28.17, 34.62, 32.0, 33.67, 36.66, 38.37, 37.5, 40.48, 40.0,
    41.18, 41.67, 43.33, 43.75, 43.33, 45.83, 45.83, 45.45, 45.0, 44.44,
    50.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0, NaN
  ];
  const single_q3 = [
    6.73, 6.86, 7.0, 8.16, 9.38, 10.42, 9.78, 14.13, 18.89, 17.05, 22.09, 15.12,
    25.0, 25.64, 27.63, 30.0, 31.25, 30.88, 30.66, 34.85, 35.82, 35.82,
    37.1, 39.66, 39.19, 42.52, 42.0, 43.11, 43.48, 47.13, 45.45, 47.5,
    47.37, 47.37, 50.0, 50.0, 50.0, 53.33, 53.85, 54.17, 54.55, 55.0,
    56.25, 57.14, 57.14, 58.33, 60.0, 62.5, 66.67, 75.0, NaN
  ];

  const dual_q1 = [
    NaN, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    3.12, 0.0, 0.0, 0.0, 2.5, 7.14, 9.52, 6.52, 12.64, 4.0, 28.21, 18.52,
    23.28, 48.71, 55.17, 56.67, 60.94, 65.15, 64.71, 61.43, 70.27, 69.44,
    71.05, 73.75, 73.75, 74.39, 76.74, 75.58, 78.89, 78.89, 80.43, 79.79,
    80.21, 81.63, 80.0, 81.73
  ];
  const dual_q2 = [
    NaN, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.33, 6.25, 11.11, 15.0, 9.09, 20.42, 17.26,
    18.93, 33.33, 31.25, 32.35, 31.07, 47.37, 50.0, 51.14, 56.67, 57.58,
    56.39, 64.41, 63.39, 65.52, 68.41, 71.67, 75.0, 73.44, 75.76, 75.76,
    76.47, 77.78, 79.17, 78.95, 80.77, 81.25, 82.93, 84.52, 83.33, 84.09,
    84.44, 85.87, 84.04, 85.42, 85.71, 85.0, 86.54
  ];
  const dual_q3 = [
    NaN, 0.0, 0.0, 33.33, 25.0, 20.0, 20.0, 41.67, 35.71, 33.33, 44.44, 35.0,
    51.04, 53.85, 60.18, 53.57, 62.92, 61.76, 63.89, 67.5, 72.5, 71.43,
    70.45, 74.73, 75.75, 76.92, 76.92, 77.39, 79.31, 81.67, 80.65, 81.25,
    83.33, 83.82, 83.82, 84.29, 86.11, 85.53, 86.25, 86.25, 87.8, 88.1,
    88.37, 88.64, 89.36, 89.36, 89.36, 88.54, 89.8, 89.0, 90.38
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
      { label: "% 2x" },
      {
        label: "1x 25%",
        fill: "rgba(0, 255, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "1x 75%",
        fill: "rgba(0, 255, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "1x Med",
        dash: [1, 5],
        points: {show: false},
        value: (_, v) => v + "%",
      },
      {
        label: "2x 25%",
        fill: "rgba(255, 0, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "2x 75%",
        fill: "rgba(255, 0, 0, .07)",
        value: (_, v) => v + "%",
        band: true,
        width: 0,
        points: {show: false},
      },
      {
        label: "2x Med",
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
        label: "Infection Rate",
        values: (_, vals) => vals.map(v => v + "%")
      },
    ]
  };

  uplot.default(
    opts, data,
    document.getElementById("infection_rate_by_household_type_vs_pct_dual_shopper"));
})();

