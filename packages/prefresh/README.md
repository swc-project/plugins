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

## 5.0.1

### Patch Changes

- a73664c: Update swc_core to v6.0.2

## 5.0.0

### Major Changes

- 4ad7f56: Update swc_core to v5

## 4.0.0

### Major Changes

- ba13397: Update swc_core to v4

## 3.0.4

### Patch Changes

- 0508b6d: Update swc_core to v3

## 3.0.3

### Patch Changes

- cd5ad2a: Update swc_core to 1.0

## 3.0.2

### Patch Changes

- 20162c8: Update swc_core to v0.106.0

## 3.0.1

### Patch Changes

- 04548e2: Update swc_core to 0.103.x

## 3.0.0

### Major Changes

- f8e5fd0: Update swc_core to 0.102.x

## 2.0.10

### Patch Changes

- 7d17e25: Update swc_core to v0.101.x

## 2.0.9

### Patch Changes

- 7391419: Update swc_core to v0.100.0

## 2.0.8

### Patch Changes

- 9c28afb: Update swc_core to 0.99.x (@swc/core 1.7.0)
