# @swc/plugin-swc-magic

`@swc/plugin-swc-magic` processes magic annotations from the `@swc/magic` package, such as `markAsPure`, which annotates expressions so the SWC minifier can eliminate them as dead code during tree-shaking.

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

${CHANGELOG}
