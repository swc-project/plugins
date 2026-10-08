---
"@swc/plugin-styled-components": patch
---

Pass `css` prop interpolations such as `${SIZES[size].bottom}` through as props when a key further along the member chain is a local, instead of hoisting them out of the component's scope.
