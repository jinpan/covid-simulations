import * as wasm from "wasm-engine";
import * as THREE from 'three';
import * as uplot from 'uplot';
import css from 'uplot/dist/uPlot.min.css'

window._wasm = wasm;
window._uplot = uplot;

let uplot_opts = {
  width: 600,
  height: 100,
  scales: {
    x: { time: false },
    y: { range: [0, 200] },
  },
  axes: [
    { show: false },
    { show: false },
  ],
  cursor: { show: false },
  legend: { show: false },
  series: [
    {},
    {
      stroke: "gray",
      fill: "rgba(200,200,200,1)",
      points: { show: false }
    },
    {
      stroke: "green",
      fill: "rgba(184,247,191,1)",
      points: { show: false }
    },
    {
      stroke: "red",
      fill: "rgba(235,99,131,1)",
      points: { show: false }
    },
    {
      stroke: "yellow",
      fill: "rgba(199,186,41,1)",
      points: { show: false }
    },
  ],
};

// For the first simulation
(function() {
  let config0 = {
    "engine_config": {
      "disease_parameters": {
        "exposed_period_ticks": 0 * 60,
        "infectious_period_ticks": 10 * 60,

        "spread_parameters": {
          "infection_radius": 3,
        },
      },

      "behavior_parameters": "brownian_motion",

      "size": [600, 400],
      "num_people": 200,
      "num_initially_infected": 1,
    },
    "map_name": "",
  };

  let config1 = {
    "engine_config": {
      "disease_parameters": {
        "exposed_period_ticks": 10 * 60,
        "infectious_period_ticks": 10 * 60,

        "spread_parameters": {
          "background_viral_particle": {
            "exhale_radius": 9,
            "exhale_amount": 0.4,
            "decay_rate": 0.05,
            "infection_risk_per_particle": 0.004,
          },
        },
      },

      "behavior_parameters": "brownian_motion",

      "size": [600, 400],
      "num_people": 200,
      "num_initially_infected": 1,
    },
    "map_name": "",
  };

  let config2 = {
    "engine_config": {
      "disease_parameters": {
        "exposed_period_ticks": 10 * 60,
        "infectious_period_ticks": 30 * 60,

        "spread_parameters": {
          "background_viral_particle": {
            "exhale_radius": 9,
            "exhale_amount": 0.4,
            "decay_rate": 0.05,
            "infection_risk_per_particle": 0.0008,
          },
        },
      },

      "behavior_parameters": "shopper",

      "size": [600, 400],
      "num_people": 108,
      "num_initially_infected": 1,
    },
    "map_name": "simple_groceries",
  };

  let config = config1;
  let world = wasm.create_world(config.engine_config, config.map_name);
  window._WORLD = world;

  let color_map = {
    "susceptible": 0xB8F7BF,
    "exposed": 0xC7BA29,
    "infectious": 0xEB6383,
    "recovered": 0xC8C8C8,
  };

  let xs = Array.from(Array(600).keys());

  let uplot_data = [
    xs,
    [], // susceptible
    [], // exposed + susceptible
    [], // exposed + infectious + susceptible
    [], // exposed + recovered + infectious + susceptible
  ];
  uplot_opts['scales']['y']['range'] = [0, config['engine_config']['num_people']];

  let uplot_inst = new uplot.default(
    uplot_opts, uplot_data,
    document.getElementById("main-plot"));

  let scene = new THREE.Scene();

  if (config.map_name != "") {
    // Draw bounding boxes for households and stores
    let box_material = new THREE.MeshBasicMaterial();

    for (const box of world.get_bounding_boxes()) {
      let width = box.right - box.left;
      let height = box.bot - box.top;

      let plane_geo = new THREE.PlaneGeometry(width, height, 1);
      let plane = new THREE.Mesh(plane_geo, box_material);

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

  // Draw people
  let circle_geo = new THREE.CircleGeometry( 4, 32 );

  let circlesByID = new Map();
  for (const person of world.to_json()["people"]) {
    let color = color_map[person.ds];
    let material = new THREE.MeshBasicMaterial( { color: color } );
    let circle = new THREE.Mesh( circle_geo, material );

    circle.position.x = person.px;
    circle.position.y = person.py;
    circle.color = new THREE.Color(person.c);

    circlesByID.set(person.id, circle);
    scene.add(circle);
  }

  const width = 600;
  const height = 400;
  const pad = 1000;
  let camera = new THREE.OrthographicCamera(
      0, width, height, 0,
      0, 1000,
  );
  camera.position.z = 5;

  let size = width * height;
  let background_color_data = new Uint8Array( 3 * size );

  for ( let i = 0; i < size; i ++ ) {
    let stride = i * 3;

    background_color_data[ stride ] = Math.floor(250);
    background_color_data[ stride + 1] = Math.floor(250);
    background_color_data[ stride + 2] = Math.floor(250);
  }
  let texture = new THREE.DataTexture( background_color_data, width, height, THREE.RGBFormat );
  scene.background = texture;

  let canvas = document.getElementById("main-canvas");
  let renderer = new THREE.WebGLRenderer({ "canvas": canvas });
  renderer.sortObjects = false;
  renderer.setClearColor (0xf0f0f0, 1);
  renderer.render( scene, camera );

  let play = false;
  let speed = 1;

  let last_tick_div_10 = 0;

  function animate() {
    if (!play) {
      return;
    }

    for (let i = 0; i < speed; i++) {
      world.step();
    }

    let state = world.to_json();

    // Update the people
    for (const person of state["people"]) {
      let circle = circlesByID.get(person.id);

      circle.position.x = person.px;
      circle.position.y = person.py;

      let color = color_map[person.ds];
      circle.material.color.setHex(color);
    }

    let tick_div_10 = Math.floor(state.tick / 10);
    if (tick_div_10 > last_tick_div_10) {
      last_tick_div_10 = tick_div_10;
      let counts = {
        "susceptible": 0,
        "exposed": 0,
        "infectious": 0,
        "recovered": 0,
      };

      for (const person of state["people"]) {
        counts[person.ds] += 1;
      }

      uplot_data[1].push(counts["exposed"] + counts["recovered"] + counts["susceptible"] + counts["infectious"]);
      uplot_data[2].push(counts["exposed"] + counts["susceptible"] + counts["infectious"]);
      uplot_data[3].push(counts["exposed"] + counts["infectious"]);
      uplot_data[4].push(counts["exposed"]);
      if (uplot_data[1].length > 600) {
        uplot_data[1].shift();
        uplot_data[2].shift();
        uplot_data[3].shift();
        uplot_data[4].shift();
      }
      uplot_inst.setData(uplot_data);
    }

    // Update the background viral particle levels.
    if ('background_viral_particle' in config['engine_config']['disease_parameters']['spread_parameters']) {
      // let background_viral_particles = world.get_background_viral_particles();
      // Directly create a Float32Array view here from the wasm buffer to avoid allocating a copy.
      // Profiles show significantly less memory allocator pressure from this.
      let background_viral_particles = new Float32Array(
        wasm._WASM_MEMORY.buffer,
        world.get_background_viral_particles2(),
        width * height,
      );

      for (let idx = 0; idx < width * height; idx++) {
        let stride = idx * 3;
        let val = background_viral_particles[idx];
        background_color_data[stride] = Math.min(255, 250 + val);
        background_color_data[stride+1] = Math.max(0, 250 - 20*val);
        background_color_data[stride+2] = Math.max(0, 250 - 20*val);
      }
      texture.needsUpdate = true;
    }

    requestAnimationFrame( animate );
    renderer.render( scene, camera );
  }
  animate();

  let start_btn = document.getElementById("main-canvas-start");
  start_btn.addEventListener("click", function() {
    if (play) {
      play = false;
      start_btn.innerText = "Start";
      return;
    }

    play = true;
    start_btn.innerText = "Pause";
    animate();
  });

  let reset_btn = document.getElementById("main-canvas-reset");
  reset_btn.addEventListener("click", function() {
    world.free();

    world = wasm.create_world(config.engine_config, config.map_name);
    window._WORLD = world;

    uplot_data = [
      xs,
      [], [], [], [],
    ];
    last_tick_div_10 = 0;

    if (!play) {
      play = true;
      start_btn.innerText = "Pause";
      animate();
    }
  });

  for (let btn of document.getElementsByClassName("main-canvas-speed")) {
    btn.addEventListener("click", function() {
      speed = parseInt(this.dataset.speed);

      for (let btn2 of document.getElementsByClassName("main-canvas-speed")) {
        btn2.style["font-weight"] = "normal";
        btn2.disabled = false;
      }
      this.style["font-weight"] = "bold";
      this.disabled = true;
    });
  };
})();

