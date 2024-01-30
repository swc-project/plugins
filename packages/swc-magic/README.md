# @swc/plugin-swc-magic

## Usage

.swcrc:

```json
{
  "jsc": {
    "experimental": {
      "plugins": ["@swc/plugin-swc-magic"]
    }
  }
}
```

### `markAsPure`

```js
import { markAsPure } from "@swc/magic";

markAsPure(() => console.log("This will be removed by the SWC minifier"));
```

# @swc/plugin-swc-magic

## 1.5.114

### Patch Changes

- 4ef0b7f: Add changelog to the readme

## 1.5.113

### Patch Changes

- 4e72680: swc_core@0.88.0

## 1.5.112

### Patch Changes

- 16bb4d8: swc_core@0.82.x
