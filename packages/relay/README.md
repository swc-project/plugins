### `@swc/plugin-relay`

#### Setup

```sh
npm install --save-dev @swc/plugin-relay @swc/core
```

### Example

The below shows how to configure `@swc/plugin-relay` and pass the options to Webpack:

Create an `.swcrc.js` file like the below:

```js
// .swcrc.js

module.exports = {
  jsc: {
    experimental: {
      plugins: [
        [
          "@swc/plugin-relay",
          {
            rootDir: __dirname,
            artifactDirectory: "src/__generated__",
            language: "typescript",
            eagerEsModules: true,
          },
        ],
      ],
    },
    parser: {
      syntax: "typescript",
      tsx: true,
    },
    transform: {
      react: {
        runtime: "automatic",
      },
    },
  },
};
```

And then update your `swc-loader` Webpack config:

```js
const swcConfig = require("./.swcrc.js")

// ...

{
  include: path.resolve("./src"),
  test: /\.ts$/,
  use: [
    {
      loader: "swc-loader",
      options: swcConfig,
    },
  ],
}
```

> Note: We're using a `.swcrc.js` file extension up above and importing the config directly because Relay needs access to `__dirname`, which can't be derived from the default JSON parsed from `.swcrc`.

#### Output import paths

By default, `@swc/plugin-relay` will transpile import paths based on the `language` option.
You can use `outputFileExtension` to change the file extension of the generated import paths.

```js
plugins: [
    [
        "@swc/plugin-relay",
        {
            rootDir: __dirname,
            artifactDirectory: "src/__generated__",
            language: "typescript",
            eagerEsModules: true,
            outputFileExtension: "js",
        },
    ],
],
```

In this example typescript graphql files will output transpiled import path of `javascript` ending with `.js`.

# @swc/plugin-relay

## 2.0.5

### Patch Changes

- 0f38844: Publish all chanages

## 2.0.4

### Patch Changes

- 1379d24: Make config parsing typed.

## 2.0.3

### Patch Changes

- 1cc9eda: Update dependencies

## 2.0.2

### Patch Changes

- 247cca6: Update rustc to 'nightly-2024-04-16'

## 2.0.1

### Patch Changes

- 876bbce: Update swc_core to 0.92.x

## 2.0.0

### Major Changes

- 8e91d39: Update swc_core to 0.91.x

## 1.6.0

### Minor Changes

- 8c6f890: Always add current directory to relative output paths

## 1.5.122

### Patch Changes

- f4df366: Update swc_core

## 1.5.121

### Patch Changes

- c88b22b: Align package metadata

## 1.5.120

### Patch Changes

- a3cc4fb: Organize pacakge metadata

## 1.5.119

### Patch Changes

- e9e78ef: Update swc crates

## 1.5.118

### Patch Changes

- 6096d6d: Fix plugin version schema issue

## 1.5.117

### Patch Changes

- 37d3aaf: Depend on the swc download counter package

## 1.5.116

### Patch Changes

- dc0dc6b: Use correct identifier for imports

## 1.5.115

### Patch Changes

- 8bd92c7: swc_core 0.90.x

## 1.5.114

### Patch Changes

- 4ef0b7f: Add changelog to the readme

## 1.5.113

### Patch Changes

- 4e72680: swc_core@0.88.0

## 1.5.112

### Patch Changes

- 16bb4d8: swc_core@0.82.x
