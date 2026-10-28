---
"@swc/plugin-styled-components": patch
---

Reduce CSS minification startup cost by scanning line breaks and symbol whitespace without compiling regular expressions for each Wasm instance.
