# @swc/plugin-swc-experimental-inject-imports

## Usage

.swcrc:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        "@swc/plugin-swc-experimental-inject-imports",
        {
          "importsPaths": ["@swc/example-import"],
          "onlyFilenames": ["page.tsx", "layout.tsx"]
        }
      ]
    }
  }
}
```

${CHANGELOG}
