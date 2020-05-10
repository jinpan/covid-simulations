import * as wasm from "wasm-engine";
import * as THREE from 'three';

// For the first simulation
(function() {
  let world = wasm.create_world({
    "disease_parameters": {
      "infectious_period_ticks": 10 * 60 ,

      "spread_parameters": {
        // "infection_radius": 18,
        "background_viral_particle": {
          "exhale_radius": 9,
          "exhale_amount": 0.2,
          "decay_rate": 0.05,
          "inhale_radius": 5,
          "infection_risk_per_particle": 0.00004,
        }
      },
    },
    "size": [600, 400],
    "num_people": 200,
    "num_initially_infected": 1,
  });

  let color_map = {
    "susceptible": 0xa8e6cf,
    "infectious": 0xeb6383,
    "recovered": 0xdcedc1,
  };

  let scene = new THREE.Scene();

  let circle_geo = new THREE.CircleGeometry( 3, 8 );

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
  const pad = 0;
  let camera = new THREE.OrthographicCamera(
      0 - pad, width + pad, height + pad, 0 - pad,
      1, 1000);
  camera.position.z = 5;

  let size = width * height;
  let background_color_data = new Uint8Array( 3 * size );

  for ( let i = 0; i < size; i ++ ) {
    let stride = i * 3;

    background_color_data[ stride ] = Math.floor(200);
    background_color_data[ stride + 1] = Math.floor(200);
    background_color_data[ stride + 2] = Math.floor(200);
  }
  let texture = new THREE.DataTexture( background_color_data, width, height, THREE.RGBFormat );
  scene.background = texture;

  let canvas = document.getElementById("main-canvas");
  let renderer = new THREE.WebGLRenderer({ "canvas": canvas });
  renderer.setClearColor (0xf0f0f0, 1);

  function animate() {
    world.step();

    let state = world.to_json();

    // Update the people
    for (const person of state["people"]) {
      let circle = circlesByID.get(person.id);

      circle.position.x = person.px;
      circle.position.y = person.py;

      let color = color_map[person.ds];
      circle.material.color.setHex(color);
    }

    // Update the background viral particle levels
    for (const [row_index, row] of state["background_viral_particles"].entries()) {
      for (const [col_index, val] of row.entries()) {
        // let stride = (col_index * width + row_index) * 3;
        let stride = (row_index * width + col_index) * 3;

        background_color_data[stride] = Math.min(255, 200 + 20 * val);
        texture.needsUpdate = true;
      }
    }


    requestAnimationFrame( animate );
    renderer.render( scene, camera );
  }
  animate();
})();

