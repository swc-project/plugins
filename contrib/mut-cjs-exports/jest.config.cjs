const path = require("node:path");

const plugin = path.resolve("./swc_mut_cjs_exports_debug.wasm");

module.exports = {
  transform: {
    "^.+\\.(t|j)sx?$": [
      "@swc/jest",
      {
        jsc: {
          experimental: {
            plugins: [[plugin, {}]],
          },
        },
        module: {
          type: "commonjs",
        },
      },
    ],
  },
};
