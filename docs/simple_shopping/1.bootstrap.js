(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "../engine/pkg sync recursive":
/*!**************************!*\
  !*** ../engine/pkg sync ***!
  \**************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function webpackEmptyContext(req) {\n\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\te.code = 'MODULE_NOT_FOUND';\n\tthrow e;\n}\nwebpackEmptyContext.keys = function() { return []; };\nwebpackEmptyContext.resolve = webpackEmptyContext;\nmodule.exports = webpackEmptyContext;\nwebpackEmptyContext.id = \"../engine/pkg sync recursive\";\n\n//# sourceURL=webpack:///../engine/pkg_sync?");

/***/ }),

/***/ "../engine/pkg/engine.js":
/*!*******************************!*\
  !*** ../engine/pkg/engine.js ***!
  \*******************************/
/*! exports provided: create_world, WorldView, __wbindgen_json_serialize, __wbindgen_json_parse, __wbindgen_object_drop_ref, __wbg_getRandomValues_f5e14ab7ac8e995d, __wbg_randomFillSync_d5bd2d655fdf256a, __wbg_self_1b7a39e3a92c949c, __wbg_require_604837428532a733, __wbg_crypto_968f1772287e2df0, __wbindgen_is_undefined, __wbg_getRandomValues_a3d34b4fee3c2869, __wbindgen_throw, _WASM_MEMORY */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./engine_bg.wasm */ \"../engine/pkg/engine_bg.wasm\");\n/* harmony import */ var _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./engine_bg.js */ \"../engine/pkg/engine_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"create_world\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"create_world\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"WorldView\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"WorldView\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_serialize\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_json_serialize\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_parse\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_json_parse\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_f5e14ab7ac8e995d\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_f5e14ab7ac8e995d\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d5bd2d655fdf256a\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_randomFillSync_d5bd2d655fdf256a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_1b7a39e3a92c949c\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_1b7a39e3a92c949c\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_604837428532a733\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_require_604837428532a733\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_968f1772287e2df0\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_crypto_968f1772287e2df0\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_a3d34b4fee3c2869\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_a3d34b4fee3c2869\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"_WASM_MEMORY\", function() { return _engine_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"_WASM_MEMORY\"]; });\n\n\n\n\n//# sourceURL=webpack:///../engine/pkg/engine.js?");

/***/ }),

/***/ "../engine/pkg/engine_bg.js":
/*!**********************************!*\
  !*** ../engine/pkg/engine_bg.js ***!
  \**********************************/
/*! exports provided: create_world, WorldView, __wbindgen_json_serialize, __wbindgen_json_parse, __wbindgen_object_drop_ref, __wbg_getRandomValues_f5e14ab7ac8e995d, __wbg_randomFillSync_d5bd2d655fdf256a, __wbg_self_1b7a39e3a92c949c, __wbg_require_604837428532a733, __wbg_crypto_968f1772287e2df0, __wbindgen_is_undefined, __wbg_getRandomValues_a3d34b4fee3c2869, __wbindgen_throw, _WASM_MEMORY */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"create_world\", function() { return create_world; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"WorldView\", function() { return WorldView; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_serialize\", function() { return __wbindgen_json_serialize; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_parse\", function() { return __wbindgen_json_parse; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_f5e14ab7ac8e995d\", function() { return __wbg_getRandomValues_f5e14ab7ac8e995d; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d5bd2d655fdf256a\", function() { return __wbg_randomFillSync_d5bd2d655fdf256a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_1b7a39e3a92c949c\", function() { return __wbg_self_1b7a39e3a92c949c; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_604837428532a733\", function() { return __wbg_require_604837428532a733; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_968f1772287e2df0\", function() { return __wbg_crypto_968f1772287e2df0; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_a3d34b4fee3c2869\", function() { return __wbg_getRandomValues_a3d34b4fee3c2869; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"_WASM_MEMORY\", function() { return _WASM_MEMORY; });\n/* harmony import */ var _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./engine_bg.wasm */ \"../engine/pkg/engine_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nlet cachegetFloat32Memory0 = null;\nfunction getFloat32Memory0() {\n    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetFloat32Memory0 = new Float32Array(_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetFloat32Memory0;\n}\n\nfunction getArrayF32FromWasm0(ptr, len) {\n    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);\n}\n\nlet stack_pointer = 32;\n\nfunction addBorrowedObject(obj) {\n    if (stack_pointer == 1) throw new Error('out of js stack');\n    heap[--stack_pointer] = obj;\n    return stack_pointer;\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n/**\n* @param {any} config\n* @param {string} map_name\n* @param {number | undefined} maybe_seed\n* @returns {WorldView}\n*/\nfunction create_world(config, map_name, maybe_seed) {\n    try {\n        var ptr0 = passStringToWasm0(map_name, _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        var ret = _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"create_world\"](addBorrowedObject(config), ptr0, len0, !isLikeNone(maybe_seed), isLikeNone(maybe_seed) ? 0 : maybe_seed);\n        return WorldView.__wrap(ret);\n    } finally {\n        heap[stack_pointer++] = undefined;\n    }\n}\n\nfunction handleError(f) {\n    return function () {\n        try {\n            return f.apply(this, arguments);\n\n        } catch (e) {\n            _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n        }\n    };\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n/**\n*/\nclass WorldView {\n\n    static __wrap(ptr) {\n        const obj = Object.create(WorldView.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_worldview_free\"](ptr);\n    }\n    /**\n    * @returns {number}\n    */\n    step() {\n        var ret = _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"worldview_step\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @returns {any}\n    */\n    to_json() {\n        var ret = _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"worldview_to_json\"](this.ptr);\n        return takeObject(ret);\n    }\n    /**\n    * @returns {Float32Array}\n    */\n    get_background_viral_particles() {\n        _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"worldview_get_background_viral_particles\"](8, this.ptr);\n        var r0 = getInt32Memory0()[8 / 4 + 0];\n        var r1 = getInt32Memory0()[8 / 4 + 1];\n        var v0 = getArrayF32FromWasm0(r0, r1).slice();\n        _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 4);\n        return v0;\n    }\n    /**\n    * @returns {number}\n    */\n    get_background_viral_particles2() {\n        var ret = _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"worldview_get_background_viral_particles2\"](this.ptr);\n        return ret;\n    }\n    /**\n    * @returns {any}\n    */\n    get_households() {\n        var ret = _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"worldview_get_households\"](this.ptr);\n        return takeObject(ret);\n    }\n    /**\n    * @returns {any}\n    */\n    get_roads() {\n        var ret = _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"worldview_get_roads\"](this.ptr);\n        return takeObject(ret);\n    }\n    /**\n    * @returns {any}\n    */\n    get_stores() {\n        var ret = _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"worldview_get_stores\"](this.ptr);\n        return takeObject(ret);\n    }\n}\n\nconst __wbindgen_json_serialize = function(arg0, arg1) {\n    const obj = getObject(arg1);\n    var ret = JSON.stringify(obj === undefined ? null : obj);\n    var ptr0 = passStringToWasm0(ret, _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbindgen_json_parse = function(arg0, arg1) {\n    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbg_getRandomValues_f5e14ab7ac8e995d = function(arg0, arg1, arg2) {\n    getObject(arg0).getRandomValues(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbg_randomFillSync_d5bd2d655fdf256a = function(arg0, arg1, arg2) {\n    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbg_self_1b7a39e3a92c949c = handleError(function() {\n    var ret = self.self;\n    return addHeapObject(ret);\n});\n\nconst __wbg_require_604837428532a733 = function(arg0, arg1) {\n    var ret = __webpack_require__(\"../engine/pkg sync recursive\")(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbg_crypto_968f1772287e2df0 = function(arg0) {\n    var ret = getObject(arg0).crypto;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_is_undefined = function(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbg_getRandomValues_a3d34b4fee3c2869 = function(arg0) {\n    var ret = getObject(arg0).getRandomValues;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nvar _WASM_MEMORY = _engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"];\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../engine/pkg/engine_bg.js?");

/***/ }),

/***/ "../engine/pkg/engine_bg.wasm":
/*!************************************!*\
  !*** ../engine/pkg/engine_bg.wasm ***!
  \************************************/
/*! exports provided: memory, __wbg_worldview_free, worldview_step, worldview_to_json, worldview_get_background_viral_particles, worldview_get_background_viral_particles2, worldview_get_households, worldview_get_roads, worldview_get_stores, create_world, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free, __wbindgen_exn_store */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./engine_bg.js */ \"../engine/pkg/engine_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../engine/pkg/engine_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_engine__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm-engine */ \"../engine/pkg/engine.js\");\n/* harmony import */ var three__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! three */ \"./node_modules/three/build/three.module.js\");\n/* harmony import */ var uplot__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! uplot */ \"./node_modules/uplot/dist/uPlot.esm.js\");\n/* harmony import */ var uplot_dist_uPlot_min_css__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! uplot/dist/uPlot.min.css */ \"./node_modules/uplot/dist/uPlot.min.css\");\n/* harmony import */ var uplot_dist_uPlot_min_css__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(uplot_dist_uPlot_min_css__WEBPACK_IMPORTED_MODULE_3__);\n/* harmony import */ var three_examples_fonts_helvetiker_bold_typeface_json__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! three/examples/fonts/helvetiker_bold.typeface.json */ \"./node_modules/three/examples/fonts/helvetiker_bold.typeface.json\");\nvar three_examples_fonts_helvetiker_bold_typeface_json__WEBPACK_IMPORTED_MODULE_4___namespace = /*#__PURE__*/__webpack_require__.t(/*! three/examples/fonts/helvetiker_bold.typeface.json */ \"./node_modules/three/examples/fonts/helvetiker_bold.typeface.json\", 1);\n\n\n\n\n\n\nlet THREE_default_font = (new three__WEBPACK_IMPORTED_MODULE_1__[\"FontLoader\"]()).parse(three_examples_fonts_helvetiker_bold_typeface_json__WEBPACK_IMPORTED_MODULE_4__);\n\nwindow._wasm = wasm_engine__WEBPACK_IMPORTED_MODULE_0__;\nwindow._uplot = uplot__WEBPACK_IMPORTED_MODULE_2__;\n\nconst width = 600;\nconst height = 400;\n\nconst color_map = {\n  \"susceptible\": 0xB8F7BF,\n  \"exposed\": 0xC7BA29,\n  \"infectious\": 0xEB6383,\n  \"recovered\": 0xC8C8C8,\n};\nconst uplot_opts = {\n  width: width, height: 50,\n  scales: { x: { time: false }, y: { range: [0, 200] }, },\n  axes: [ { show: false }, { show: false }, ],\n  cursor: { show: false },\n  legend: { show: false },\n  series: [\n    {},\n    { stroke: \"gray\", fill: \"rgba(200,200,200,1)\", points: { show: false } },\n    { stroke: \"green\", fill: \"rgba(184,247,191,1)\", points: { show: false } },\n    { stroke: \"red\", fill: \"rgba(235,99,131,1)\", points: { show: false } },\n    { stroke: \"yellow\", fill: \"rgba(199,186,41,1)\", points: { show: false } },\n  ],\n};\n\nclass Simulation {\n  constructor(config) {\n    this.config = Object.assign({}, config);  // Deep copy of the config\n    this.world = wasm_engine__WEBPACK_IMPORTED_MODULE_0__[\"create_world\"](config.engine_config, config.map_name, config.initial_seed);\n\n    this._play = false;\n    this.speed = 1;\n\n    this.reset_uplot(this.config);\n    this.reset_three(this.config, this.world);\n\n    this.register_buttons();\n  }\n\n  reset_uplot() {\n    let opts_copy = Object.assign({}, uplot_opts);\n    opts_copy['scales']['y']['range'] = [0, this.config['engine_config']['num_people']];\n    this.uplot_data = [\n      Array.from(Array(width).keys()),\n      [], // susceptible\n      [], // exposed + susceptible\n      [], // exposed + infectious + susceptible\n      [], // exposed + recovered + infectious + susceptible\n    ];\n    if (this.uplot_inst === undefined) {\n      this.uplot_inst = new uplot__WEBPACK_IMPORTED_MODULE_2__[\"default\"](\n        opts_copy, this.uplot_data,\n        document.getElementById(`${this.config.name}-uplot`),\n      );\n    } else {\n      // A uplot instance has been previously created.\n      // Reset the data for it.\n      this.uplot_inst.setData(this.uplot_data);\n    }\n    this.next_chart_update_tick = 0;\n  }\n\n  reset_three() {\n    // The garbage collector will take care of deleting the old scene.\n    this.scene = new three__WEBPACK_IMPORTED_MODULE_1__[\"Scene\"]();\n\n    if (this.config.map_name != \"\") {\n      this.draw_map(this.world, this.scene);\n    }\n\n    this.draw_people(this.world, this.scene);\n    this.draw_background(this.world, this.scene);\n\n    this.camera = new three__WEBPACK_IMPORTED_MODULE_1__[\"OrthographicCamera\"](\n        0, width, height, 0,\n        0, 1000,\n    );\n    this.camera.position.z = 5;\n\n    let canvas = document.getElementById(`${this.config.name}-canvas`);\n    this.renderer = new three__WEBPACK_IMPORTED_MODULE_1__[\"WebGLRenderer\"]({ \"canvas\": canvas });\n    this.renderer.sortObjects = false;\n    this.renderer.setClearColor (0xfafafa, 1);\n    this.renderer.render(this.scene, this.camera);\n  }\n\n  draw_map(world, scene) {\n    // Draw households\n    let household_material = new three__WEBPACK_IMPORTED_MODULE_1__[\"MeshBasicMaterial\"]();\n    let household_text_material = new three__WEBPACK_IMPORTED_MODULE_1__[\"MeshBasicMaterial\"]({\n      \"color\": 0xAAAAAA, \"side\": three__WEBPACK_IMPORTED_MODULE_1__[\"DoubleSide\"],\n      \"transparent\": true, \"opacity\": 0.4,\n    });\n    for (const household of world.get_households()) {\n      let box = household.bounds;\n      let width = box.right - box.left;\n      let height = box.bot - box.top;\n\n      let plane_geo = new three__WEBPACK_IMPORTED_MODULE_1__[\"PlaneGeometry\"](width, height, 1);\n      let plane = new three__WEBPACK_IMPORTED_MODULE_1__[\"Mesh\"](plane_geo, household_material);\n\n      plane.position.x = (box.left + box.right) / 2;\n      plane.position.y = (box.bot + box.top) / 2;\n\n      let plane_box = new three__WEBPACK_IMPORTED_MODULE_1__[\"BoxHelper\"](plane, 0x000000);\n      scene.add(plane_box);\n\n      let msg = (household.dual_shopper) ? \"2x\" : \"1x\";\n      let text_geo = new three__WEBPACK_IMPORTED_MODULE_1__[\"TextGeometry\"](msg, {\n        \"font\": THREE_default_font,\n        \"size\": 15,\n      });\n      let text = new three__WEBPACK_IMPORTED_MODULE_1__[\"Mesh\"](text_geo, household_text_material);\n\n      text.position.x = box.left + 5;\n      text.position.y = box.top + 2;\n\n      scene.add(text);\n    }\n\n    // Draw stores\n    let store_material = new three__WEBPACK_IMPORTED_MODULE_1__[\"MeshBasicMaterial\"]();\n    for (const box of world.get_stores()) {\n      let width = box.right - box.left;\n      let height = box.bot - box.top;\n\n      let plane_geo = new three__WEBPACK_IMPORTED_MODULE_1__[\"PlaneGeometry\"](width, height, 1);\n      let plane = new three__WEBPACK_IMPORTED_MODULE_1__[\"Mesh\"](plane_geo, store_material);\n\n      plane.position.x = (box.left + box.right) / 2;\n      plane.position.y = (box.bot + box.top) / 2;\n\n      let plane_box = new three__WEBPACK_IMPORTED_MODULE_1__[\"BoxHelper\"](plane, 0x000000);\n      scene.add(plane_box);\n    }\n\n    // Draw roads\n    let road_material = new three__WEBPACK_IMPORTED_MODULE_1__[\"MeshBasicMaterial\"]({\n      \"color\": 0x333333,\n    });\n    for (const box of world.get_roads()) {\n      let width = box.right - box.left;\n      let height = box.bot - box.top;\n\n      let plane_geo = new three__WEBPACK_IMPORTED_MODULE_1__[\"PlaneGeometry\"](width, height, 1);\n      let plane = new three__WEBPACK_IMPORTED_MODULE_1__[\"Mesh\"](plane_geo, road_material);\n\n      plane.position.x = (box.left + box.right) / 2;\n      plane.position.y = (box.bot + box.top) / 2;\n\n      scene.add(plane);\n    }\n  }\n\n  draw_people(world, scene) {\n    let circle_geo = new three__WEBPACK_IMPORTED_MODULE_1__[\"CircleGeometry\"]( 4, 32 );\n\n    this.people_by_id = new Map();\n    for (const person_state of world.to_json()[\"people\"]) {\n      let color = color_map[person_state[\"ds\"]];\n      let material = new three__WEBPACK_IMPORTED_MODULE_1__[\"MeshBasicMaterial\"]( { color: color } );\n      let person = new three__WEBPACK_IMPORTED_MODULE_1__[\"Mesh\"]( circle_geo, material );\n\n      person.position.x = person_state[\"px\"];\n      person.position.y = person_state[\"py\"];\n\n      this.people_by_id.set(person_state[\"id\"], person);\n      scene.add(person);\n    }\n  }\n\n  draw_background(world, scene) {\n    let size = width * height;\n    this.background_color_data = new Uint8Array( 3 * size );\n\n    for ( let i = 0; i < size; i ++ ) {\n      let stride = i * 3;\n\n      this.background_color_data[ stride ] = Math.floor(250);\n      this.background_color_data[ stride + 1] = Math.floor(250);\n      this.background_color_data[ stride + 2] = Math.floor(250);\n    }\n    this.texture = new three__WEBPACK_IMPORTED_MODULE_1__[\"DataTexture\"]( this.background_color_data, width, height, three__WEBPACK_IMPORTED_MODULE_1__[\"RGBFormat\"] );\n    scene.background = this.texture;\n  }\n\n  animate_people(people) {\n    for (const person_state of people) {\n      let person = this.people_by_id.get(person_state[\"id\"]);\n\n      person.position.x = person_state[\"px\"];\n      person.position.y = person_state[\"py\"];\n\n      let color = color_map[person_state[\"ds\"]];\n      person.material.color.setHex(color);\n    }\n  }\n\n  update_chart(people) {\n    let counts = {\n      \"susceptible\": 0,\n      \"exposed\": 0,\n      \"infectious\": 0,\n      \"recovered\": 0,\n    };\n    for (const person of people) {\n      counts[person.ds] += 1;\n    }\n\n    const a = counts[\"exposed\"];\n    const b = a + counts[\"infectious\"];\n    const c = b + counts[\"susceptible\"];\n    const d = c + counts[\"recovered\"];\n\n    this.uplot_data[1].push(d);\n    this.uplot_data[2].push(c);\n    this.uplot_data[3].push(b);\n    this.uplot_data[4].push(a);\n\n    if (this.uplot_data[1].length > width) {\n      this.uplot_data[1].shift();\n      this.uplot_data[2].shift();\n      this.uplot_data[3].shift();\n      this.uplot_data[4].shift();\n    }\n    this.uplot_inst.setData(this.uplot_data);\n  }\n\n  update_background_viral_particles() {\n    // let background_viral_particles = world.get_background_viral_particles();\n    // Directly create a Float32Array view here from the wasm buffer to avoid allocating a copy.\n    // Profiles show significantly less memory allocator pressure from this.\n    const background_viral_particles = new Float32Array(\n      wasm_engine__WEBPACK_IMPORTED_MODULE_0__[\"_WASM_MEMORY\"].buffer,\n      this.world.get_background_viral_particles2(),\n      width * height,\n    );\n\n    for (let idx = 0; idx < width * height; idx++) {\n      const stride = idx * 3;\n      const val = background_viral_particles[idx];\n      this.background_color_data[stride] = Math.min(255, 250 + val);\n      this.background_color_data[stride+1] = Math.max(0, 250 - 8*val);\n      this.background_color_data[stride+2] = Math.max(0, 250 - 8*val);\n    }\n    this.texture.needsUpdate = true;\n  }\n\n  animate() {\n    if (!this._play) {\n      return;\n    }\n    requestAnimationFrame( () => {this.animate();} );\n\n    let state = null;\n    for (let i = 0; i < this.speed; i++) {\n      const tick = this.world.step();\n      if (tick > this.next_chart_update_tick) {\n        this.next_chart_update_tick += this.config[\"chart_update_period_ticks\"];\n\n        state = this.world.to_json();\n        this.update_chart(state[\"people\"]);\n      }\n    }\n    if (state == null) {\n      state = this.world.to_json();\n    }\n\n    this.animate_people(state[\"people\"]);\n    let spread_params = this.config[\"engine_config\"][\"disease_parameters\"][\"spread_parameters\"];\n    if ('background_viral_particle' in spread_params) {\n      this.update_background_viral_particles();\n    }\n\n\n    this.renderer.render( this.scene, this.camera );\n  }\n\n  // Controller related utilities\n  play() {\n    const was_paused = !this._play;\n    this._play = true;\n\n    if (was_paused) {\n      // Pause other simulations.\n      for (let i = 0; i < simulations.length; i++) {\n        let sim = simulations[i];\n        if (sim != this) {\n          sim.pause();\n        }\n      }\n\n      let start_btn = document.getElementById(`${this.config.name}-start`);\n      start_btn.innerText = \"Pause\";\n      this.animate();\n    }\n  }\n  pause() {\n    this._play = false;\n    let start_btn = document.getElementById(`${this.config.name}-start`);\n    start_btn.innerText = \"Start\";\n  }\n\n  reset() {\n    this.world.free();\n    // Only the first world is deterministically seeded.\n    // Subsequent worlds are randomly seeded.\n    this.world = wasm_engine__WEBPACK_IMPORTED_MODULE_0__[\"create_world\"](this.config.engine_config, this.config.map_name);\n\n    this.reset_uplot();\n    this.reset_three();\n  }\n\n  register_buttons() {\n    let sim = this;\n    const cfg_name = sim.config.name;\n\n    // Start/Pause button\n    let start_btn = document.getElementById(`${cfg_name}-start`);\n    start_btn.addEventListener(\"click\", function() {\n      if (sim._play) {\n        sim.pause();\n      } else {\n        sim.play();\n      }\n    });\n\n    // Reset button\n    let reset_btn = document.getElementById(`${cfg_name}-reset`);\n    reset_btn.addEventListener(\"click\", function() {\n      sim.reset();\n    });\n\n    // Speed buttons\n    for (let btn of document.getElementsByClassName(`${cfg_name}-speed`)) {\n      btn.addEventListener(\"click\", function() {\n        // Update button appearances\n        for (let btn2 of document.getElementsByClassName(`${cfg_name}-speed`)) {\n          btn2.style[\"font-weight\"] = \"normal\";\n          btn2.disabled = false;\n        }\n        this.style[\"font-weight\"] = \"bold\";\n        this.disabled = true;\n\n        sim.speed = parseInt(this.dataset.speed);\n      });\n    };\n\n    for (let btn of document.getElementsByClassName(`${cfg_name}-pct-dual-shopper`)) {\n      btn.addEventListener(\"click\", function() {\n        // Update button appearances\n        for (let btn2 of document.getElementsByClassName(`${cfg_name}-pct-dual-shopper`)) {\n          btn2.style[\"font-weight\"] = \"normal\";\n          btn2.disabled = false;\n        }\n        this.style[\"font-weight\"] = \"bold\";\n        this.disabled = true;\n\n        const fraction_dual_shopper = parseInt(this.dataset.pct) / 100;\n        let shopper_params = sim.config['engine_config']['behavior_parameters']['shopper'];\n        shopper_params['fraction_dual_shopper_households'] = fraction_dual_shopper;\n\n        sim.reset();\n      });\n    };\n  }\n}\n\nconst configs = (function() {\n  let config_builder = {};\n\n  let add_config = (name, data) => {\n    data[\"name\"] = name;\n    config_builder[name] = data;\n  };\n\n  add_config(\"radius_brownian0\", {\n    \"engine_config\": {\n      \"disease_parameters\": {\n        \"exposed_period_ticks\": 0 * 60,\n        \"infectious_period_ticks\": 345,\n        \"spread_parameters\": {\n          \"infection_radius\": 3.2,\n        },\n      },\n      \"behavior_parameters\": \"brownian_motion\",\n      \"size\": [width, height],\n      \"num_people\": 200,\n      \"num_initially_infected\": 3,\n    },\n    \"map_name\": \"\",\n    \"initial_seed\": 10914,\n    \"chart_update_period_ticks\": 4,\n  });\n  add_config(\"particle_brownian0\", {\n    \"engine_config\": {\n      \"disease_parameters\": {\n        \"exposed_period_ticks\": 115,\n        \"infectious_period_ticks\": 345,\n        \"spread_parameters\": {\n          \"background_viral_particle\": {\n            \"exhale_radius\": 9,\n            \"decay_rate\": 0.05,\n            \"infection_risk_per_particle\": 0.0019,\n          },\n        },\n      },\n      \"behavior_parameters\": \"brownian_motion\",\n      \"size\": [width, height],\n      \"num_people\": 200,\n      \"num_initially_infected\": 3,\n    },\n    \"map_name\": \"\",\n    \"initial_seed\": 10914,\n    \"chart_update_period_ticks\": 6,\n  });\n  add_config(\"particle_shopper0\", {\n    \"engine_config\": {\n      \"disease_parameters\": {\n        \"exposed_period_ticks\": 15 * 60,\n        \"infectious_period_ticks\": 45 * 60,\n        \"spread_parameters\": {\n          \"background_viral_particle\": {\n            \"exhale_radius\": 9,\n            \"decay_rate\": 0.055,\n            \"infection_risk_per_particle\": 0.00013,\n          },\n        },\n      },\n      \"behavior_parameters\": {\n        \"shopper\": {\n          \"fraction_dual_shopper_households\": 0.5,\n          \"shopping_period_ticks\": 10 * 60,\n          \"supplies_bought_per_trip\": 30 * 60,\n        },\n      },\n      \"size\": [width, height],\n      \"num_people\": 108,\n      \"num_initially_infected\": 2,\n    },\n    \"map_name\": \"simple_groceries\",\n    \"initial_seed\": 10914,\n    \"chart_update_period_ticks\": 30,\n  });\n\n  return config_builder;\n})();\n\nconst simulations = (function() {\n  let simulations_builder = [];\n\n  for (const config_name in configs) {\n    simulations_builder.push(new Simulation(configs[config_name]));\n  }\n\n  return simulations_builder;\n})();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);