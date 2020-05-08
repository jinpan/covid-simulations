import * as wasm from "wasm-engine";

// TODO: remove this debugging hack.
window.WASM_ENGINE = wasm;

var world = wasm.create_world({
  "disease_parameters": {
    "infection_radius": 18,
      "infectious_period_ticks": 10 * 60 ,
  },
  "size": [600, 400],
  "num_people": 200,
  "num_initially_infected": 1,
});
window._WORLD = world;

import * as THREE from 'three';
window.THREE = THREE;

var scene = new THREE.Scene();

var geometry = new THREE.CircleGeometry( 3, 8 );

let circlesByID = new Map();
window._CID = circlesByID;
for (const person of world.to_json()["people"]) {
  let material = new THREE.MeshBasicMaterial( { color: person.color } );
  let circle = new THREE.Mesh( geometry, material );

  circle.position.x = person.position_x;
  circle.position.y = person.position_y;
  // circle.color = new THREE.Color(person.color);

  circlesByID.set(person.id, circle);
  scene.add(circle);
}

const width = 600;
const height = 400;
const pad = 5;
var camera = new THREE.OrthographicCamera(
  0 - pad, width + pad, height + pad, 0 - pad,
  1, 1000);
camera.position.z = 5;

let canvas = document.getElementById("main-canvas");
var renderer = new THREE.WebGLRenderer({ "canvas": canvas });
renderer.setClearColor (0xf0f0f0, 1);

function animate() {
  world.step();

  for (const person of world.to_json()["people"]) {
    let circle = circlesByID.get(person.id);

    circle.position.x = person.position_x;
    circle.position.y = person.position_y;
    circle.material.color.setHex(person.color);
    // circle.material.color.setHex(0x0000ff);
  }

  requestAnimationFrame( animate );
  renderer.render( scene, camera );
}
animate();
