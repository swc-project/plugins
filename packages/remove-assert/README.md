# @swc/plugin-remove-assert

SWC plugin for removing assert statements in production builds.

## Description

This plugin removes `assert()` calls and `assert.*()` method calls (like `assert.ok()`, `assert.equal()`, etc.) from your code during the build process. This is useful for reducing bundle size and removing debug assertions in production builds.

## Installation

```bash
npm install --save-dev @swc/plugin-remove-assert
```

## Usage

Add the plugin to your `.swcrc` configuration:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@swc/plugin-remove-assert", {}]
      ]
    }
  }
}
```

## Configuration

### Remove all assert statements (default)

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        "@swc/plugin-remove-assert"
      ]
    }
  }
}
```

### Exclude specific assert methods

You can exclude specific assert methods from being removed:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@swc/plugin-remove-assert",
          {
            "exclude": ["fail"]
          }
        ]
      ]
    }
  }
}
```

With the above configuration, `assert.fail()` calls will be preserved while other assert calls will be removed.

## Example

**Input:**

```javascript
import assert from 'assert';

function divide(a, b) {
  assert(b !== 0, 'Division by zero');
  return a / b;
}

assert.equal(1, 1);
assert.ok(true);
```

**Output:**

```javascript
import assert from 'assert';

function divide(a, b) {
  return a / b;
}
```

## License

Apache-2.0
