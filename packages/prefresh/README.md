# @swc/plugin-prefresh

The SWC implementation of the [prefresh babel plugin](https://github.com/preactjs/prefresh/tree/main/packages/babel).

## Usage

Prefresh babel plugin is a forked equivalent of the react-refresh babel plugin difference being that we need a way to memoize createContext between HMR.

And SWC has built-in React Refresh transformation, therefore, this plugin only implements the `createContext` processing part and need to be used with `jsc.transform.react.refresh`.

.swcrc:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          // enable prefresh specific transformation
          "@swc/plugin-prefresh",
          {
            // the customizable preact name, default is `["preact", "preact/compat", "react"]`
            "library": ["preact-like-framework"]
          }
        ]
      ]
    },
    "parser": {
      "jsx": true
    },
    "transform": {
      "react": {
        "development": true,
        "refresh": true, // enable react refresh transformation
      }
    }
  }
}
```

# @swc/plugin-prefresh

## 2.0.9

### Patch Changes

- 7391419: Update swc_core to v0.100.0

## 2.0.8

### Patch Changes

- 9c28afb: Update swc_core to 0.99.x (@swc/core 1.7.0)
