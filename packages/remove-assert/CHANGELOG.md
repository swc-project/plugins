# Changelog

## 0.1.0 (2026-02-06)

### Features

* Initial release of `@swc/plugin-remove-assert`
* Remove assert statements in production builds
* Support for both direct `assert()` calls and method calls like `assert.strictEqual()`
* Configurable `exclude` option to keep specific assert methods
* Compatible with Node.js assert module and similar libraries
