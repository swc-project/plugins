---
"@swc-contrib/plugin-graphql-codegen-client-preset": patch
"@swc/plugin-emotion": patch
"@swc/plugin-styled-components": patch
---

Reuse existing boxed expressions instead of allocating new boxes, fixing `clippy::replace_box` warnings without changing transform behavior.
