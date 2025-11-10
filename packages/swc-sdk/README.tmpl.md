# @swc/plugin-swc-sdk

# Usage

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

## `/*#__DYNAMIC__*/` on imports

You can annotate on a normal import statement to make it dynamic import.

### Input

```js
/*#__DYNAMIC__*/
import { log } from "./logger";

export async function work() {
  if (process.env.NODE_ENV !== "production") {
    await log("Starting work");
  }

  if (process.env.NODE_ENV !== "production") {
    log("Without await");
  }
}

export function sync() {
  if (process.env.NODE_ENV !== "production") {
    log("Without async");
  }
}
```

### Output

```js
export function work() {
  if (process.env.NODE_ENV !== "production") {
    await import('./logger').then(m => m.log("Starting work"))
  }

  if (process.env.NODE_ENV !== "production") {
    import('./logger').then(m => m.log("Without await"))
  }
}

export function sync() {
  if (process.env.NODE_ENV !== "production") {
    import('./logger').then(m => m.log("Without async"))
  }
}
```

## `markAsPure`

```js
import { markAsPure } from "@swc/sdk";

markAsPure(() => console.log("This will be removed by the SWC minifier"));
```

${CHANGELOG}
