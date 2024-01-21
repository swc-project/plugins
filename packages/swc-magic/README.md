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

```js
import { markAsPure } from "@swc/magic";

markAsPure(() => console.log("This will be removed by the SWC minifier"));
```

${CHANGELOG}
