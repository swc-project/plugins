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
            language: "typescript",
            schema: "data/schema.graphql",
            rootDir: __dirname,
            src: "src",
            artifactDirectory: "src/__generated__",
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
