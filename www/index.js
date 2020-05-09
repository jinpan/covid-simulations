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
          "exhale_amount": 1,
          "decay_rate": 0.5,
          "inhale_radius": 9,
          "infection_risk_per_particle": 0.0002,
        }
      },
    },
    "size": [600, 400],
    "num_people": 200,
    "num_initially_infected": 1,
  });

  let scene = new THREE.Scene();

  let circle_geo = new THREE.CircleGeometry( 3, 8 );

  let circlesByID = new Map();
  for (const person of world.to_json()["people"]) {
    let material = new THREE.MeshBasicMaterial( { color: person.color } );
    let circle = new THREE.Mesh( circle_geo, material );

    circle.position.x = person.px;
    circle.position.y = person.py;
    circle.color = new THREE.Color(person.c);

    circlesByID.set(person.id, circle);
    scene.add(circle);
  }

  const width = 600;
  const height = 400;
  const pad = 5;
  let camera = new THREE.OrthographicCamera(
      0 - pad, width + pad, height + pad, 0 - pad,
      1, 1000);
  camera.position.z = 5;

  let canvas = document.getElementById("main-canvas");
  let renderer = new THREE.WebGLRenderer({ "canvas": canvas });
  renderer.setClearColor (0xf0f0f0, 1);

  function animate() {
    world.step();

    for (const person of world.to_json()["people"]) {
      let circle = circlesByID.get(person.id);

      circle.position.x = person.px;
      circle.position.y = person.py;
      circle.material.color.setHex(person.c);
    }

    requestAnimationFrame( animate );
    renderer.render( scene, camera );
  }
  animate();

})();

