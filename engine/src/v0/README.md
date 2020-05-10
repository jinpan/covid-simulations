# V0 Simulation Engine

List of things the v0 engine currently does not handle well:
* Physical Units
    * Distance is abstract
    * Time is abstract (defined in ticks)
* 3 dimensions
    * Everything is simulated on a flat surface
* Maps are abstract
* The rendering loop is slow:
    * Current flow is wasm_view -> json -> three.js
    * Could be faster with rendering webgl from the wasm side
