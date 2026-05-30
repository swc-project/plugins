---
"@swc/plugin-formatjs": patch
---

Avoid hygiene proxy crashes by evaluating only static formatjs descriptor values.
The legacy Rust visitor constructor now ignores its evaluator argument; use
`create_formatjs_visitor_without_evaluator` for new integrations.
