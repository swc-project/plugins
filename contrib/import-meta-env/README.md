# `@swc-contrib/plugin-import-meta-env`

Simple SWC plugin to transform `import.meta.env` to `process.env`.

This package is based on [`swc-plugin-import-meta-env`](https://github.com/Codex-/swc-plugin-import-meta-env) and is maintained under the SWC contrib plugin workspace.

## Installation

```bash
npm i -D @swc-contrib/plugin-import-meta-env
```

## Usage

Add the plugin to the `jsc.experimental.plugins` field of your `.swcrc`.

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["@swc-contrib/plugin-import-meta-env", {}]]
    }
  }
}
```

The plugin only rewrites the `import.meta.env` expression shape. It does not load or expand environment files, so populate `process.env` through your test setup, runtime, or another tool such as `dotenv`.

# @swc-contrib/plugin-import-meta-env

## 1.15.41

### Minor Changes

- Add `@swc-contrib/plugin-import-meta-env`, based on `swc-plugin-import-meta-env`.
