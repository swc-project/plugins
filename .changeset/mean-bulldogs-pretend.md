---
"@swc/plugin-emotion": patch
---

Fix the emotion `keyframes` auto-label regression so generated animations keep the plain name string instead of receiving a `label:` CSS fragment.
