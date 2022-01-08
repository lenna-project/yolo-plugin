"use strict";
(self["webpackChunkyolo_plugin"] = self["webpackChunkyolo_plugin"] || []).push([[23],{

/***/ 23:
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "__wbg_error_09919627ac0992f5": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.gk),
/* harmony export */   "__wbg_new_693216e109162396": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.Ih),
/* harmony export */   "__wbg_stack_0ddaca5d1abfb52f": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.yq),
/* harmony export */   "__wbindgen_json_parse": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.t$),
/* harmony export */   "__wbindgen_json_serialize": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.r1),
/* harmony export */   "__wbindgen_object_drop_ref": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.ug),
/* harmony export */   "defaultConfig": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.u_),
/* harmony export */   "description": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.WL),
/* harmony export */   "id": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.id),
/* harmony export */   "name": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.u2),
/* harmony export */   "process": () => (/* reexport safe */ _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__.N4)
/* harmony export */ });
/* harmony import */ var _yolo_plugin_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(86);



/***/ }),

/***/ 86:
/***/ ((module, __webpack_exports__, __webpack_require__) => {

/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "u_": () => (/* binding */ defaultConfig),
/* harmony export */   "id": () => (/* binding */ id),
/* harmony export */   "u2": () => (/* binding */ name),
/* harmony export */   "WL": () => (/* binding */ description),
/* harmony export */   "N4": () => (/* binding */ process),
/* harmony export */   "ug": () => (/* binding */ __wbindgen_object_drop_ref),
/* harmony export */   "t$": () => (/* binding */ __wbindgen_json_parse),
/* harmony export */   "r1": () => (/* binding */ __wbindgen_json_serialize),
/* harmony export */   "Ih": () => (/* binding */ __wbg_new_693216e109162396),
/* harmony export */   "yq": () => (/* binding */ __wbg_stack_0ddaca5d1abfb52f),
/* harmony export */   "gk": () => (/* binding */ __wbg_error_09919627ac0992f5)
/* harmony export */ });
/* harmony import */ var _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(840);
/* module decorator */ module = __webpack_require__.hmd(module);


const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(_yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(_yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetInt32Memory0;
}
/**
* @returns {any}
*/
function defaultConfig() {
    var ret = _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.defaultConfig();
    return takeObject(ret);
}

/**
* @returns {string}
*/
function id() {
    try {
        const retptr = _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.id(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1);
    }
}

/**
* @returns {string}
*/
function name() {
    try {
        const retptr = _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.name(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1);
    }
}

/**
* @returns {string}
*/
function description() {
    try {
        const retptr = _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.description(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1);
    }
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
* @param {any} config
* @param {Uint8Array} data
* @returns {Uint8Array}
*/
function process(config, data) {
    try {
        const retptr = _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
        var ptr0 = passArray8ToWasm0(data, _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.process(retptr, addHeapObject(config), ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayU8FromWasm0(r0, r1).slice();
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1 * 1);
        return v1;
    } finally {
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
    }
}

function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

function __wbindgen_json_parse(arg0, arg1) {
    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

function __wbindgen_json_serialize(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = JSON.stringify(obj === undefined ? null : obj);
    var ptr0 = passStringToWasm0(ret, _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

function __wbg_new_693216e109162396() {
    var ret = new Error();
    return addHeapObject(ret);
};

function __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

function __wbg_error_09919627ac0992f5(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        _yolo_plugin_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(arg0, arg1);
    }
};



/***/ }),

/***/ 840:
/***/ ((module, __unused_webpack_exports, __webpack_require__) => {

"use strict";
// Instantiate WebAssembly module
var wasmExports = __webpack_require__.w[module.id];

// export exports from WebAssembly module
module.exports = wasmExports;
// exec imports from WebAssembly module (for esm order)
/* harmony import */ var m0 = __webpack_require__(86);


// exec wasm module
wasmExports[""]()

/***/ })

}]);
//# sourceMappingURL=23.js.map