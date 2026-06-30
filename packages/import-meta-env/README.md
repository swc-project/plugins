# @swc/plugin-import-meta-env

`@swc/plugin-import-meta-env` transforms `import.meta.env` expressions to `process.env`.

This is useful when code written for Vite-style `import.meta.env` needs to run in an environment where values are already exposed through `process.env`, such as Jest or Node-based test tooling.

## Usage

`.swcrc`:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["@swc/plugin-import-meta-env", {}]]
    }
  }
}
```

### Input

```js
const mode = import.meta.env.MODE;
const value = import.meta.env["VALUE"];
```

### Output

```js
const mode = process.env.MODE;
const value = process.env["VALUE"];
```

## Notes

This plugin only rewrites the expression shape. It does not load `.env` files, inline values, or emulate Vite environment variable filtering.

This package is based on the Apache-2.0 licensed [`swc-plugin-import-meta-env`](https://github.com/Codex-/swc-plugin-import-meta-env) implementation.

# @swc/plugin-import-meta-env

## 0.1.0

### Minor Changes

- Initial release: transform `import.meta.env` expressions to `process.env`.
