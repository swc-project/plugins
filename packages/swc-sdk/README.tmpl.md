# @swc/plugin-swc-sdk

## Usage

.swcrc:

```json
{
  "jsc": {
    "experimental": {
      "plugins": ["@swc/plugin-swc-sdk"]
    }
  }
}
```

### `markAsPure`

```js
import { markAsPure } from "@swc/sdk";

markAsPure(() => console.log("This will be removed by the SWC minifier"));
```

${CHANGELOG}
