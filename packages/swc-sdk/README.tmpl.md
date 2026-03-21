# @swc/plugin-swc-sdk

`@swc/plugin-swc-sdk` provides build-time transformations for the `@swc/sdk` package. It supports converting static imports annotated with `/*#__DYNAMIC__*/` into dynamic imports, and applies configured SDK feature-flag transforms.

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

${CHANGELOG}
