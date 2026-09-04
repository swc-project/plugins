---
"@swc/plugin-emotion": patch
---

Merge generated `target`/`label` options into non-object-literal `styled` options instead of appending a third argument.

`styled('div', someIdentifier)` and `styled('div', config.opts)` previously emitted
`styled('div', someIdentifier, { target, label })`. `@emotion/styled` only reads two
arguments, so the generated `target` was silently dropped and `${Component}` selectors
against those components broke at runtime with no build-time signal. These now spread the
original expression into the options object, matching the existing handling of call
expressions such as `styled('div', makeOptions())`.
