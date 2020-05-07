import * as wasm from "wasm-engine";

// TODO: remove this debugging hack.
window.WASM_ENGINE = wasm;

wasm.foo();

import * as THREE from 'three';
window.THREE = THREE;

var scene = new THREE.Scene();

var geometry = new THREE.CircleGeometry( 5, 32 );
var material = new THREE.MeshBasicMaterial( { color: 0xff0000 } );
var circle = new THREE.Mesh( geometry, material );
scene.add( circle );
var circle_right = true;

function animate_circle() {
  setTimeout(animate_circle, 16);
  if (circle_right) {
    if (circle.position.x < 100) {
      circle.position.x += 1;
    } else {
      circle_right = false;
    }
  } else {
    if (circle.position.x > -100) {
      circle.position.x -= 1;
    } else {
      circle_right = true;
    }
  }
}
animate_circle();

const width = 400;
const height = 300;
var camera = new THREE.OrthographicCamera(
  width / - 2, width / 2, height / 2, height / - 2,
  1, 1000);
camera.position.z = 5;

let canvas = document.getElementById("main-canvas");
var renderer = new THREE.WebGLRenderer({ "canvas": canvas });
renderer.setClearColor (0xf0f0f0, 1);

function animate() {
  requestAnimationFrame( animate );
  renderer.render( scene, camera );
}
animate();
