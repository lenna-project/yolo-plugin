"use strict";
(self["webpackChunkyolo_plugin"] = self["webpackChunkyolo_plugin"] || []).push([[138,276],{

/***/ 853:
/***/ ((module, __unused_webpack___webpack_exports__, __webpack_require__) => {

/* harmony import */ var _node_modules_css_loader_dist_runtime_cssWithMappingToString_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(15);
/* harmony import */ var _node_modules_css_loader_dist_runtime_cssWithMappingToString_js__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(_node_modules_css_loader_dist_runtime_cssWithMappingToString_js__WEBPACK_IMPORTED_MODULE_0__);
/* harmony import */ var _node_modules_css_loader_dist_runtime_api_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(645);
/* harmony import */ var _node_modules_css_loader_dist_runtime_api_js__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(_node_modules_css_loader_dist_runtime_api_js__WEBPACK_IMPORTED_MODULE_1__);
// Imports


var ___CSS_LOADER_EXPORT___ = _node_modules_css_loader_dist_runtime_api_js__WEBPACK_IMPORTED_MODULE_1___default()((_node_modules_css_loader_dist_runtime_cssWithMappingToString_js__WEBPACK_IMPORTED_MODULE_0___default()));
// Module
___CSS_LOADER_EXPORT___.push([module.id, "\n.plugin-config[data-v-7949c1db] {\n  margin: 5px;\n  min-width: 100px;\n  min-height: 50px;\n}\n", "",{"version":3,"sources":["webpack://./src/Widget.vue"],"names":[],"mappings":";AA0CA;EACE,WAAW;EACX,gBAAgB;EAChB,gBAAgB;AAClB","sourcesContent":["<template>\n  <div class=\"plugin-config\">\n    <div>\n      <input\n        type=\"checkbox\"\n        id=\"crop\"\n        v-model=\"crop\"\n        v-on:change=\"updateConfig()\"\n      />\n      <label for=\"crop\">Crop first detection</label>\n    </div>\n  </div>\n</template>\n\n<script>\nimport { defineComponent } from \"vue\";\nexport default defineComponent({\n  name: \"Widget\",\n  props: {\n    defaultConfig: Object,\n  },\n  data() {\n    return {\n      crop: false,\n    };\n  },\n  methods: {\n    async updateConfig() {\n      let config = {\n        crop: this.crop,\n      };\n      this.$emit(\"changeConfig\", config);\n    },\n  },\n  created() {\n    this.crop = this.defaultConfig.crop;\n    this.updateConfig();\n  },\n});\n</script>\n\n<style scoped lang=\"css\">\n.plugin-config {\n  margin: 5px;\n  min-width: 100px;\n  min-height: 50px;\n}\n</style>\n"],"sourceRoot":""}]);
// Exports
/* unused harmony default export */ var __WEBPACK_DEFAULT_EXPORT__ = ((/* unused pure expression or super */ null && (___CSS_LOADER_EXPORT___)));


/***/ }),

/***/ 645:
/***/ ((module) => {



/*
  MIT License http://www.opensource.org/licenses/mit-license.php
  Author Tobias Koppers @sokra
*/
// css base code, injected by the css-loader
// eslint-disable-next-line func-names
module.exports = function (cssWithMappingToString) {
  var list = []; // return the list of modules as css string

  list.toString = function toString() {
    return this.map(function (item) {
      var content = cssWithMappingToString(item);

      if (item[2]) {
        return "@media ".concat(item[2], " {").concat(content, "}");
      }

      return content;
    }).join("");
  }; // import a list of modules into the list
  // eslint-disable-next-line func-names


  list.i = function (modules, mediaQuery, dedupe) {
    if (typeof modules === "string") {
      // eslint-disable-next-line no-param-reassign
      modules = [[null, modules, ""]];
    }

    var alreadyImportedModules = {};

    if (dedupe) {
      for (var i = 0; i < this.length; i++) {
        // eslint-disable-next-line prefer-destructuring
        var id = this[i][0];

        if (id != null) {
          alreadyImportedModules[id] = true;
        }
      }
    }

    for (var _i = 0; _i < modules.length; _i++) {
      var item = [].concat(modules[_i]);

      if (dedupe && alreadyImportedModules[item[0]]) {
        // eslint-disable-next-line no-continue
        continue;
      }

      if (mediaQuery) {
        if (!item[2]) {
          item[2] = mediaQuery;
        } else {
          item[2] = "".concat(mediaQuery, " and ").concat(item[2]);
        }
      }

      list.push(item);
    }
  };

  return list;
};

/***/ }),

/***/ 15:
/***/ ((module) => {



function _slicedToArray(arr, i) { return _arrayWithHoles(arr) || _iterableToArrayLimit(arr, i) || _unsupportedIterableToArray(arr, i) || _nonIterableRest(); }

function _nonIterableRest() { throw new TypeError("Invalid attempt to destructure non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method."); }

function _unsupportedIterableToArray(o, minLen) { if (!o) return; if (typeof o === "string") return _arrayLikeToArray(o, minLen); var n = Object.prototype.toString.call(o).slice(8, -1); if (n === "Object" && o.constructor) n = o.constructor.name; if (n === "Map" || n === "Set") return Array.from(o); if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)) return _arrayLikeToArray(o, minLen); }

function _arrayLikeToArray(arr, len) { if (len == null || len > arr.length) len = arr.length; for (var i = 0, arr2 = new Array(len); i < len; i++) { arr2[i] = arr[i]; } return arr2; }

function _iterableToArrayLimit(arr, i) { var _i = arr && (typeof Symbol !== "undefined" && arr[Symbol.iterator] || arr["@@iterator"]); if (_i == null) return; var _arr = []; var _n = true; var _d = false; var _s, _e; try { for (_i = _i.call(arr); !(_n = (_s = _i.next()).done); _n = true) { _arr.push(_s.value); if (i && _arr.length === i) break; } } catch (err) { _d = true; _e = err; } finally { try { if (!_n && _i["return"] != null) _i["return"](); } finally { if (_d) throw _e; } } return _arr; }

function _arrayWithHoles(arr) { if (Array.isArray(arr)) return arr; }

module.exports = function cssWithMappingToString(item) {
  var _item = _slicedToArray(item, 4),
      content = _item[1],
      cssMapping = _item[3];

  if (!cssMapping) {
    return content;
  }

  if (typeof btoa === "function") {
    // eslint-disable-next-line no-undef
    var base64 = btoa(unescape(encodeURIComponent(JSON.stringify(cssMapping))));
    var data = "sourceMappingURL=data:application/json;charset=utf-8;base64,".concat(base64);
    var sourceMapping = "/*# ".concat(data, " */");
    var sourceURLs = cssMapping.sources.map(function (source) {
      return "/*# sourceURL=".concat(cssMapping.sourceRoot || "").concat(source, " */");
    });
    return [content].concat(sourceURLs).concat([sourceMapping]).join("\n");
  }

  return [content].join("\n");
};

/***/ }),

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

/***/ 276:
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

// ESM COMPAT FLAG
__webpack_require__.r(__webpack_exports__);

// EXPORTS
__webpack_require__.d(__webpack_exports__, {
  "default": () => (/* binding */ Widget)
});

// EXTERNAL MODULE: consume shared module (default) vue@^3.2.6 (strict) (fallback: ./node_modules/@vue/runtime-dom/dist/runtime-dom.esm-bundler.js)
var runtime_dom_esm_bundler_js_ = __webpack_require__(855);
;// CONCATENATED MODULE: ./node_modules/vue-loader/dist/templateLoader.js??ruleSet[1].rules[1]!./node_modules/vue-loader/dist/index.js??ruleSet[1].rules[5].use[0]!./src/Widget.vue?vue&type=template&id=7949c1db&scoped=true


const _withScopeId = n => ((0,runtime_dom_esm_bundler_js_.pushScopeId)("data-v-7949c1db"),n=n(),(0,runtime_dom_esm_bundler_js_.popScopeId)(),n)
const _hoisted_1 = { class: "plugin-config" }
const _hoisted_2 = /*#__PURE__*/ _withScopeId(() => /*#__PURE__*/(0,runtime_dom_esm_bundler_js_.createElementVNode)("label", { for: "crop" }, "Crop first detection", -1))

function render(_ctx, _cache, $props, $setup, $data, $options) {
  return ((0,runtime_dom_esm_bundler_js_.openBlock)(), (0,runtime_dom_esm_bundler_js_.createElementBlock)("div", _hoisted_1, [
    (0,runtime_dom_esm_bundler_js_.createElementVNode)("div", null, [
      (0,runtime_dom_esm_bundler_js_.withDirectives)((0,runtime_dom_esm_bundler_js_.createElementVNode)("input", {
        type: "checkbox",
        id: "crop",
        "onUpdate:modelValue": _cache[0] || (_cache[0] = $event => ((_ctx.crop) = $event)),
        onChange: _cache[1] || (_cache[1] = $event => (_ctx.updateConfig()))
      }, null, 544), [
        [runtime_dom_esm_bundler_js_.vModelCheckbox, _ctx.crop]
      ]),
      _hoisted_2
    ])
  ]))
}
;// CONCATENATED MODULE: ./src/Widget.vue?vue&type=template&id=7949c1db&scoped=true

;// CONCATENATED MODULE: ./node_modules/vue-loader/dist/index.js??ruleSet[1].rules[5].use[0]!./src/Widget.vue?vue&type=script&lang=js


/* harmony default export */ const Widgetvue_type_script_lang_js = ((0,runtime_dom_esm_bundler_js_.defineComponent)({
  name: "Widget",
  props: {
    defaultConfig: Object,
  },
  data() {
    return {
      crop: false,
    };
  },
  methods: {
    async updateConfig() {
      let config = {
        crop: this.crop,
      };
      this.$emit("changeConfig", config);
    },
  },
  created() {
    this.crop = this.defaultConfig.crop;
    this.updateConfig();
  },
}));

;// CONCATENATED MODULE: ./src/Widget.vue?vue&type=script&lang=js
 
// EXTERNAL MODULE: ./node_modules/css-loader/dist/cjs.js!./node_modules/vue-loader/dist/stylePostLoader.js!./node_modules/vue-loader/dist/index.js??ruleSet[1].rules[5].use[0]!./src/Widget.vue?vue&type=style&index=0&id=7949c1db&scoped=true&lang=css
var Widgetvue_type_style_index_0_id_7949c1db_scoped_true_lang_css = __webpack_require__(853);
;// CONCATENATED MODULE: ./src/Widget.vue?vue&type=style&index=0&id=7949c1db&scoped=true&lang=css

// EXTERNAL MODULE: ./node_modules/vue-loader/dist/exportHelper.js
var exportHelper = __webpack_require__(744);
;// CONCATENATED MODULE: ./src/Widget.vue




;


const __exports__ = /*#__PURE__*/(0,exportHelper/* default */.Z)(Widgetvue_type_script_lang_js, [['render',render],['__scopeId',"data-v-7949c1db"]])

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
/* harmony import */ var _Widget__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(276);
const pkg = __webpack_require__.e(/* import() */ 17).then(__webpack_require__.bind(__webpack_require__, 17));

const ui = _Widget__WEBPACK_IMPORTED_MODULE_0__["default"];
const processor = pkg;
const name = () => "yolo-plugin";
const description = () => "Yolo object detection";
const process = async (config, image) => {
  return __webpack_require__.e(/* import() */ 17).then(__webpack_require__.bind(__webpack_require__, 17)).then((processor) => processor.process(config, image));
};
const defaultConfig = async () => {
  return {
    crop: false,
  };
};


/***/ })

}]);
//# sourceMappingURL=138.js.map