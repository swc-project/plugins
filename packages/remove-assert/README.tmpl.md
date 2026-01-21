# remove-assert

SWC plugin for removing assert statements in production builds.

Similar to how Python's `-O` flag and C's `NDEBUG` macro eliminate assertions in production, this plugin removes `assert()` calls during compilation, reducing bundle size and runtime overhead.

## Config

```json
["@swc/plugin-remove-assert"]
```

## Examples

### Input
```javascript
assert(x > 0, "x must be positive");
const result = compute();
assert(result !== null, "result cannot be null");
return result;
```

### Output
```javascript
;
const result = compute();
;
return result;
```

Note: The plugin intelligently handles locally-defined `assert` functions and respects variable shadowing, only removing calls to the built-in `assert` function.

${CHANGELOG}
