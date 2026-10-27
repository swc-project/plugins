# import-meta-env

`@swc/plugin-import-meta-env` transforms `import.meta.env` to `process.env`.
This is the official SWC plugin based on [swc-plugin-import-meta-env](https://github.com/Codex-/swc-plugin-import-meta-env) by Alex Miller.

## Install

```sh
npm install --save-dev @swc/plugin-import-meta-env
```

## Usage

Add the plugin to your `.swcrc`:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["@swc/plugin-import-meta-env", {}]]
    }
  }
}
```

```js
// Input
const env = import.meta.env;
const mode = import.meta.env.MODE;
const value = import.meta.env["KEY"];

// Output
const env = process.env;
const mode = process.env.MODE;
const value = process.env["KEY"];
```

The plugin has no configuration options. It only replaces the `import.meta.env`
expression. Other expressions, such as `import.meta.url` and `import.meta["env"]`,
remain unchanged.

The runtime must provide `process.env`. The plugin does not load `.env` files,
inline environment values, or provide Vite-specific environment variables.

To migrate from `swc-plugin-import-meta-env`, install `@swc/plugin-import-meta-env`
and replace the package name in your plugin configuration.

${CHANGELOG}
