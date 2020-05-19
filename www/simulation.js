import * as wasm from "wasm-engine";
import * as THREE from 'three';
import * as uplot from 'uplot';
import 'uplot/dist/uPlot.min.css';
import THREE_default_font_json from 'three/examples/fonts/helvetiker_bold.typeface.json';

export const width = 600;
export const height = 400;

const color_map = {
  "susceptible": 0xB8F7BF,
  "exposed": 0xC7BA29,
  "infectious": 0xEB6383,
  "recovered": 0xC8C8C8,
};

const THREE_default_font = (new THREE.FontLoader()).parse(THREE_default_font_json);

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

let simulations = [];

export class Simulation {
  constructor(config) {
    this.config = Object.assign({}, config);  // Deep copy of the config
    this.world = wasm.create_world(config.engine_config, config.map_name, config.initial_seed);

    this._play = false;
    this.speed = 1;

    this.reset_uplot(this.config);
    this.reset_three(this.config, this.world);

    this.register_buttons();

    simulations.push(this);
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
    const background_viral_particles = this.world.get_background_viral_particles();

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
