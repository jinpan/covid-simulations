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

    let household_txt_material = new THREE.MeshBasicMaterial({
      "color": 0xAAAAAA, "side": THREE.DoubleSide,
      "transparent": true, "opacity": 0.4,
    });
    const single_household_txt_geo = new THREE.TextGeometry("1x", {
      "font": THREE_default_font,
      "size": 15,
    });
    const dual_household_txt_geo = new THREE.TextGeometry("2x", {
      "font": THREE_default_font,
      "size": 15,
    });

    for (const household of world.get_households()) {
      let box = household.bounds;
      let width = box.right - box.left;
      let height = box.top - box.bottom;

      let plane_geo = new THREE.PlaneGeometry(width, height, 1);
      let plane = new THREE.Mesh(plane_geo, household_material);

      plane.position.x = (box.left + box.right) / 2;
      plane.position.y = (box.bottom + box.top) / 2;

      let plane_box = new THREE.BoxHelper(plane, 0x000000);
      scene.add(plane_box);

      let txt_geo = (household.dual_shopper) ? dual_household_txt_geo : single_household_txt_geo;
      let txt = new THREE.Mesh(txt_geo, household_txt_material);

      txt.position.x = box.left + 5;
      txt.position.y = box.bottom + 2;

      scene.add(txt);
    }

    // Draw stores
    let store_material = new THREE.MeshBasicMaterial();
    for (const box of world.get_stores()) {
      let width = box.right - box.left;
      let height = box.top - box.bottom;

      let plane_geo = new THREE.PlaneGeometry(width, height, 1);
      let plane = new THREE.Mesh(plane_geo, store_material);

      plane.position.x = (box.left + box.right) / 2;
      plane.position.y = (box.bottom + box.top) / 2;

      let plane_box = new THREE.BoxHelper(plane, 0x000000);
      scene.add(plane_box);
    }

    // Draw roads
    let road_material = new THREE.MeshBasicMaterial({
      "color": 0x333333,
    });
    for (const box of world.get_roads()) {
      let width = box.right - box.left;
      let height = box.top - box.bottom;

      let plane_geo = new THREE.PlaneGeometry(width, height, 1);
      let plane = new THREE.Mesh(plane_geo, road_material);

      plane.position.x = (box.left + box.right) / 2;
      plane.position.y = (box.bottom + box.top) / 2;

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
      "bounding_box": {
        "bottom": 0,
        "left": 0,
        "top": height,
        "right": width,
      },
      "num_people": 108,
      "num_initially_infected": 2,
    },
    "map_name": "simple_groceries",
    "initial_seed": 10914,
    "chart_update_period_ticks": 30,
  });

  return config_builder;
})();

const simulations = [
  new Simulation(configs["radius_brownian0"]),
  new Simulation(configs["particle_brownian0"]),
  new Simulation(configs["particle_shopper0"]),
];

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

