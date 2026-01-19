# remove-assert

SWC plugin for removing assert statements in production builds.

Similar to how Python's `-O` flag and C's `NDEBUG` macro eliminate assertions in production, this plugin removes `assert()` calls during compilation, reducing bundle size and runtime overhead.

## Config

```json
["@swc/plugin-remove-assert"]
```

# @swc/plugin-remove-assert

## 0.1.0

### Initial Release

- Initial implementation of the remove-assert plugin for SWC
