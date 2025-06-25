# transform-ns-imports

The plugin for patch transform-imports plugin and
provide transform namespace import syntax to proxy entry for dynamic ESM.

## Configuration

```json
[
  "transform-ns-imports",
  {
    "react-icon": {
      "rewrite": "react-icon/proxy-import"
    },
    "react-component": {
      "transform": "react-component/proxy-import"
    }
  }
]
```

## Behavior

Add it transform reexport from:

```ts
export * as someModule from "rewrite-module-namespace";
```

to:

```ts
export {default as someModule} from "rewrite-module-namespace/proxy-import";
```

and transform import from:

```ts
import * as test from 'rewrite-module-namespace'
```

to:

```ts
import test from "rewrite-module-namespace/proxy-import";
```

And you can create the proxy-import file content see like:

```ts
import {lazy} from 'react'

const modules = {};
export default new Proxy(modules, {
    get(name) {
        return lazy(() => import(`module/${name}`);
    }
})
```

${CHANGELOG}
