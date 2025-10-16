# @swc/plugin-swc-inject-imports

## Usage

.swcrc:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        "@swc/plugin-swc-inject-imports",
        {
          "importsPaths": ["@swc/inject-imports"],
          "onlyFilenames": ["page.tsx", "layout.tsx"]
        }
      ]
    }
  }
}
```

# @swc/plugin-swc-inject-imports
