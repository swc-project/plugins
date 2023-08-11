# `@swc/constify`

This plugin can be used to hoist constant parts of any expressions as constant variables, without affecting the runtime behavior or readability of the code.

## Configuration

The plugin

```json
{
  "jsc": {
   "experimental": {
     "plugins": [ ["@swc/plugin-constify", {
     }] ]
   }
}
```
