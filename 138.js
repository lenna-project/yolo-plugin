"use strict";
(self["webpackChunkyolo_plugin"] = self["webpackChunkyolo_plugin"] || []).push([[138,180],{

/***/ 744:
/***/ ((__unused_webpack_module, exports) => {

var __webpack_unused_export__;

__webpack_unused_export__ = ({ value: true });
// runtime helper for setting properties on components
// in a tree-shakable way
exports.Z = (sfc, props) => {
    const target = sfc.__vccOpts || sfc;
    for (const [key, val] of props) {
        target[key] = val;
    }
    return target;
};


/***/ }),

/***/ 180:
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

// ESM COMPAT FLAG
__webpack_require__.r(__webpack_exports__);

// EXPORTS
__webpack_require__.d(__webpack_exports__, {
  "default": () => (/* binding */ Widget)
});

// EXTERNAL MODULE: consume shared module (default) vue@^3.2.6 (strict) (fallback: ./node_modules/@vue/runtime-dom/dist/runtime-dom.esm-bundler.js)
var runtime_dom_esm_bundler_js_ = __webpack_require__(855);
;// CONCATENATED MODULE: ./node_modules/vue-loader/dist/templateLoader.js??ruleSet[1].rules[1]!./node_modules/vue-loader/dist/index.js??ruleSet[1].rules[5].use[0]!./src/Widget.vue?vue&type=template&id=65e1ac4e&scoped=true


const _withScopeId = n => (_pushScopeId("data-v-65e1ac4e"),n=n(),_popScopeId(),n)
const _hoisted_1 = { class: "plugin-config" }

function render(_ctx, _cache, $props, $setup, $data, $options) {
  return ((0,runtime_dom_esm_bundler_js_.openBlock)(), (0,runtime_dom_esm_bundler_js_.createElementBlock)("div", _hoisted_1))
}
;// CONCATENATED MODULE: ./src/Widget.vue?vue&type=template&id=65e1ac4e&scoped=true

;// CONCATENATED MODULE: ./node_modules/vue-loader/dist/index.js??ruleSet[1].rules[5].use[0]!./src/Widget.vue?vue&type=script&lang=js


/* harmony default export */ const Widgetvue_type_script_lang_js = ((0,runtime_dom_esm_bundler_js_.defineComponent)({
  name: "Widget",
  props: {
    defaultConfig: Object,
  },
  data() {
    return {};
  },
  methods: {
    async updateConfig() {
      let config = {};
      this.$emit("changeConfig", config);
    },
  },
  created() {
    this.updateConfig();
  },
}));

;// CONCATENATED MODULE: ./src/Widget.vue?vue&type=script&lang=js
 
// EXTERNAL MODULE: ./node_modules/vue-loader/dist/exportHelper.js
var exportHelper = __webpack_require__(744);
;// CONCATENATED MODULE: ./src/Widget.vue




;


const __exports__ = /*#__PURE__*/(0,exportHelper/* default */.Z)(Widgetvue_type_script_lang_js, [['render',render],['__scopeId',"data-v-65e1ac4e"]])

/* harmony default export */ const Widget = (__exports__);

/***/ }),

/***/ 138:
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "ui": () => (/* binding */ ui),
/* harmony export */   "processor": () => (/* binding */ processor),
/* harmony export */   "name": () => (/* binding */ name),
/* harmony export */   "description": () => (/* binding */ description),
/* harmony export */   "process": () => (/* binding */ process),
/* harmony export */   "defaultConfig": () => (/* binding */ defaultConfig)
/* harmony export */ });
/* harmony import */ var _Widget__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(180);
const pkg = __webpack_require__.e(/* import() */ 23).then(__webpack_require__.bind(__webpack_require__, 23));

const ui = _Widget__WEBPACK_IMPORTED_MODULE_0__["default"];
const processor = pkg;
const name = () => "yolo-plugin";
const description = () => "Yolo object detection";
const process = async (config, image) => {
  return __webpack_require__.e(/* import() */ 23).then(__webpack_require__.bind(__webpack_require__, 23)).then((processor) => processor.process(config, image));
};
const defaultConfig = async () => {
  return {};
};


/***/ })

}]);
//# sourceMappingURL=138.js.map