!function(e){function n(n){for(var t,o,i=n[0],u=n[1],a=0,s=[];a<i.length;a++)o=i[a],Object.prototype.hasOwnProperty.call(r,o)&&r[o]&&s.push(r[o][0]),r[o]=0;for(t in u)Object.prototype.hasOwnProperty.call(u,t)&&(e[t]=u[t]);for(f&&f(n);s.length;)s.shift()()}var t={},r={4:0};var o={};var i={18:function(){return{"./engine_bg.js":{__wbindgen_json_parse:function(e,n){return t[8].exports.j(e,n)},__wbindgen_json_serialize:function(e,n){return t[8].exports.k(e,n)},__wbindgen_object_drop_ref:function(e){return t[8].exports.m(e)},__wbindgen_is_undefined:function(e){return t[8].exports.i(e)},__wbg_buffer_eb5185aa4a8e9c62:function(e){return t[8].exports.a(e)},__wbg_newwithbyteoffsetandlength_a31622ccc380e8b4:function(e,n,r){return t[8].exports.e(e,n,r)},__wbg_self_1b7a39e3a92c949c:function(){return t[8].exports.h()},__wbg_require_604837428532a733:function(e,n){return t[8].exports.g(e,n)},__wbg_crypto_968f1772287e2df0:function(e){return t[8].exports.b(e)},__wbg_getRandomValues_a3d34b4fee3c2869:function(e){return t[8].exports.c(e)},__wbg_getRandomValues_f5e14ab7ac8e995d:function(e,n,r){return t[8].exports.d(e,n,r)},__wbg_randomFillSync_d5bd2d655fdf256a:function(e,n,r){return t[8].exports.f(e,n,r)},__wbindgen_throw:function(e,n){return t[8].exports.n(e,n)},__wbindgen_memory:function(){return t[8].exports.l()}}}}};function u(n){if(t[n])return t[n].exports;var r=t[n]={i:n,l:!1,exports:{}};return e[n].call(r.exports,r,r.exports,u),r.l=!0,r.exports}u.e=function(e){var n=[],t=r[e];if(0!==t)if(t)n.push(t[2]);else{var a=new Promise((function(n,o){t=r[e]=[n,o]}));n.push(t[2]=a);var s,c=document.createElement("script");c.charset="utf-8",c.timeout=120,u.nc&&c.setAttribute("nonce",u.nc),c.src=function(e){return u.p+"./"+({}[e]||e)+".bundle.js"}(e);var f=new Error;s=function(n){c.onerror=c.onload=null,clearTimeout(l);var t=r[e];if(0!==t){if(t){var o=n&&("load"===n.type?"missing":n.type),i=n&&n.target&&n.target.src;f.message="Loading chunk "+e+" failed.\n("+o+": "+i+")",f.name="ChunkLoadError",f.type=o,f.request=i,t[1](f)}r[e]=void 0}};var l=setTimeout((function(){s({type:"timeout",target:c})}),12e4);c.onerror=c.onload=s,document.head.appendChild(c)}return({1:[18]}[e]||[]).forEach((function(e){var t=o[e];if(t)n.push(t);else{var r,a=i[e](),s=fetch(u.p+""+{18:"ddfe9c8eb7bd9538d2e1"}[e]+".module.wasm");if(a instanceof Promise&&"function"==typeof WebAssembly.compileStreaming)r=Promise.all([WebAssembly.compileStreaming(s),a]).then((function(e){return WebAssembly.instantiate(e[0],e[1])}));else if("function"==typeof WebAssembly.instantiateStreaming)r=WebAssembly.instantiateStreaming(s,a);else{r=s.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,a)}))}n.push(o[e]=r.then((function(n){return u.w[e]=(n.instance||n).exports})))}})),Promise.all(n)},u.m=e,u.c=t,u.d=function(e,n,t){u.o(e,n)||Object.defineProperty(e,n,{enumerable:!0,get:t})},u.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},u.t=function(e,n){if(1&n&&(e=u(e)),8&n)return e;if(4&n&&"object"==typeof e&&e&&e.__esModule)return e;var t=Object.create(null);if(u.r(t),Object.defineProperty(t,"default",{enumerable:!0,value:e}),2&n&&"string"!=typeof e)for(var r in e)u.d(t,r,function(n){return e[n]}.bind(null,r));return t},u.n=function(e){var n=e&&e.__esModule?function(){return e.default}:function(){return e};return u.d(n,"a",n),n},u.o=function(e,n){return Object.prototype.hasOwnProperty.call(e,n)},u.p="",u.oe=function(e){throw console.error(e),e},u.w={};var a=window.webpackJsonp=window.webpackJsonp||[],s=a.push.bind(a);a.push=n,a=a.slice();for(var c=0;c<a.length;c++)n(a[c]);var f=s;u(u.s=2)}({2:function(e,n,t){Promise.all([t.e(0),t.e(1),t.e(8)]).then(t.bind(null,6)).catch(e=>console.error("Error importing `shopping_solo.js`:",e))}});