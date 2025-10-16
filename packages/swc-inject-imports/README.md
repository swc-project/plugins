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
          "importsPaths": ["@swc/experimental-inject-imports"],
          "onlyFilenames": ["page.tsx", "layout.tsx"]
        }
      ]
    }
  }
}
```

# @swc/plugin-swc-experimental-inject-imports
