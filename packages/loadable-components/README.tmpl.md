# loadable-components

## Config

```json
["loadable-components", {}]
```

Sometimes you need to wrap loadable with your own custom logic. There are many use cases for it, from injecting telemetry to hiding external libraries behind facade.
By default `loadable-components` are configured to transform dynamic imports used only inside loadable helpers, but can be configured to instrument any other function of your choice.

### Custom signatures with specific package sources

```json
["loadable-components", { "signatures": [
    {
        "from": "myLoadableWrapper",
        "name": "default"
    },
    {
        "from": "myLoadableWrapper",
        "name": "lazy"
    }]
}]
```

### Custom signatures without package source (matches any import)

To match any import with a specific name regardless of the source package (useful for vendored or aliased packages), omit the `from` field:

```json
["loadable-components", { "signatures": [
    {
        "name": "loadable"
    }]
}]
```

This will transform any default import named `loadable`, regardless of where it's imported from:
```js
import loadable from 'my-vendored-loadable';  // will be transformed
import loadable from '@loadable/component';   // will be transformed
```

${CHANGELOG}
