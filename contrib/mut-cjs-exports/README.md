# [SWC plugin] mutable CJS exports

[![Crates.io](https://img.shields.io/crates/v/swc_mut_cjs_exports)](https://crates.io/crates/swc_mut_cjs_exports)
[![npm](https://img.shields.io/npm/v/swc_mut_cjs_exports)](https://www.npmjs.com/package/swc_mut_cjs_exports)

[![Test](https://github.com/magic-akari/swc_mut_cjs_exports/actions/workflows/test.yml/badge.svg)](https://github.com/magic-akari/swc_mut_cjs_exports/actions/workflows/test.yml)
[![SWC Compat Test](https://github.com/magic-akari/swc_mut_cjs_exports/actions/workflows/swc-compat-test.yml/badge.svg)](https://github.com/magic-akari/swc_mut_cjs_exports/actions/workflows/swc-compat-test.yml)
[![with @swc/core@latest](https://github.com/magic-akari/swc_mut_cjs_exports/actions/workflows/cron-latest.yml/badge.svg)](https://github.com/magic-akari/swc_mut_cjs_exports/actions/workflows/cron-latest.yml)
[![with @swc/core@nightly](https://github.com/magic-akari/swc_mut_cjs_exports/actions/workflows/cron-nightly.yml/badge.svg)](https://github.com/magic-akari/swc_mut_cjs_exports/actions/workflows/cron-nightly.yml)

This is a SWC plugin to emit mutable CJS exports.

This SWC plugin has only been tested for compatibility with jest. It should be used with `@swc/jest`.

This project was previously called jest_workaround

## plugin version

https://swc.rs/docs/plugin/selecting-swc-core

## usage

install

```bash
npm i -D jest @swc/core @swc/jest swc_mut_cjs_exports
```

```js
// jest.config.js
const fs = require("node:fs");

const swcrc = JSON.parse(fs.readFileSync(".swcrc", "utf8"));

// If you have other plugins, change this line.
((swcrc.jsc ??= {}).experimental ??= {}).plugins = [
  ["swc_mut_cjs_exports", {}],
];

module.exports = {
  transform: {
    "^.+\\.(t|j)sx?$": ["@swc/jest", swcrc],
  },
};
```

Alternative implementation without .swcrc file

```JavaScript
// jest.config.js

module.exports = {
  transform: {
    "^.+\\.(t|j)sx?$": [
      "@swc/jest",
      {
        jsc: {
          experimental: {
            plugins: [["swc_mut_cjs_exports", {}]],
          },
        },
      },
    ],
  },
};
```

Make sure that `module.type` is `commonjs` in your `.swcrc` since this plugin
does not touch non-workaround parts, such as import statements.

## FAQ

#### 1. When do I need this?

If you're using the swc compiler to transform your code to comply with the ESM
specification, but you're also using Jest to test it in a CJS environment, you
may encounter issues due to the immutable issue of `exports`.

This plugin can help by transforming the `export` statements into mutable
`exports`.

#### 2. Do I have a better choice?

You may have other options depending on your specific needs:

- If you're able to run Jest in an ESM environment, you can use swc to transpile
  TypeScript/JSX syntax or downgrade JavaScript syntax without module
  conversion. Simply set the value of `module.type` to `es6` to achieve this.

- It's possible that some issues related to running Jest in an ESM environment
  will be resolved over time. Keep an eye on
  [facebook/jest#9430](https://github.com/facebook/jest/issues/9430) for
  updates.

- If you don't need the behavior of ESM specifically, you can stick with the CJS
  syntax to get the CJS behavior of `exports`.

These options may be worth considering before using this plugin.

CJS syntax

```JavaScript
exports.foo = function foo() {
  return 42;
};
```

CTS(CJS in TypeScript) syntax

```TypeScript
export = {
  foo: function foo() {
    return 42;
  },
};
```

Notes:

- ESM style export means immutable exports when transformed into CJS
- ESM style import means hoisted require when transformed into CJS

#### 3. After upgrading the plugin version, the changes have not taken effect.

This is a known issue. You could remove the Jest cache by running
[`jest --clearCache`](https://jestjs.io/docs/cli#--clearcache) as a workaround.
