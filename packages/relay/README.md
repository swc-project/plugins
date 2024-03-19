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

#### Outputting paths with './'

Default behaviour is to output import paths without a current directory qualifier. Use `add_current_directory_to_path` to always add it.

```js
plugins: [
    [
        "@swc/plugin-relay",
        {
            rootDir: __dirname,
            language: "typescript",
            eagerEsModules: true,
            addCurrentDirectoryToPath: true,
        },
    ],
],
```

# @swc/plugin-relay

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
