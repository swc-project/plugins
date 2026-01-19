---
"@swc/plugin-remove-assert": minor
---

Add remove-assert plugin for removing assert statements in production builds.

**What:** New Wasm plugin that automatically removes `assert()` statements during compilation, reducing bundle size and runtime overhead in production builds.

**Why:** Similar to Python's `-O` flag and C's `NDEBUG` macro, assertions are useful during development but add unnecessary overhead in production. This plugin provides an easy way to strip them out during the build process.

**How to use:**
```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@swc/plugin-remove-assert"]
      ]
    }
  }
}
```

Features:
- Automatic removal of global `assert()` calls
- Preserves locally-defined `assert` functions (scoping aware)
- Fully tested with comprehensive test cases