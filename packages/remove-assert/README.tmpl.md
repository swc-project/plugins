# remove-assert

SWC plugin to remove assert statements in production builds, similar to Python's `-O` flag or C's `NDEBUG`.

## Usage

### Basic usage - removes all assert statements

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

### With exclusions - keep specific assert methods

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@swc/plugin-remove-assert",
          {
            "exclude": ["strictEqual", "deepEqual"]
          }
        ]
      ]
    }
  }
}
```

## Examples

### Input

```javascript
import assert from 'assert';

function divide(a, b) {
  assert(b !== 0, 'Division by zero');
  assert.strictEqual(typeof a, 'number');
  return a / b;
}
```

### Output (default config)

```javascript
import assert from 'assert';

function divide(a, b) {
  return a / b;
}
```

### Output (with exclude: ["strictEqual"])

```javascript
import assert from 'assert';

function divide(a, b) {
  assert.strictEqual(typeof a, 'number');
  return a / b;
}
```

${CHANGELOG}
