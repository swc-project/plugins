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

## 9.3.0

### Minor Changes

- 47be132: Update swc_core to v42

## 9.2.0

### Minor Changes

- 0c9d7a1: build: Update swc_core to v39.0.0

## 9.1.0

### Minor Changes

- a872100: build: Update swc_core to v38

## 9.0.3

### Patch Changes

- 9cdcdc5: Update swc_core to v36

## 9.0.2

### Patch Changes

- 9b08ff7: Update swc_core to v35

## 9.0.1

### Patch Changes

- df280ad: Update swc_core to v34.0.0

## 9.0.0

### Major Changes

- 562e755: Update swc_core to v33

## 8.0.4

### Patch Changes

- 85719ca: Update swc_core to v32

## 8.0.3

### Patch Changes

- 45d1ac7: Update swc_core to v31

## 8.0.2

### Patch Changes

- 54febbc: Update swc_core to v29

## 8.0.1

### Patch Changes

- 8d5ce5c: Update swc_core to v28.0.0

## 8.0.0

### Major Changes

- cf2636b: Update swc_core to v27

## 7.0.5

### Patch Changes

- e3e743d: Update swc_core to v27

## 7.0.4

### Patch Changes

- 5ddbaeb: Update swc_core to v23

## 7.0.3

### Patch Changes

- d51d525: Update swc_core to v22.0.0

## 7.0.2

### Patch Changes

- cb94b92: Update swc_core to v21.0.1

## 7.0.1

### Patch Changes

- 31e3254: build: Update `swc_core` to `v19.0.0`

## 7.0.0

### Major Changes

- f0fee1d: Update swc_core to v15.0.1

## 6.3.2

### Patch Changes

- 04465bc: Update swc_core to v14.0.0, really

## 6.3.1

### Patch Changes

- ce8d317: Update swc_core to v14.0.0

## 6.3.0

### Minor Changes

- bfa0a51: Update swc_core to v13

## 6.2.0

### Minor Changes

- b8c4e6c: Update swc_core to v12

## 6.1.0

### Minor Changes

- 4c8b0e2: Update swc_core

## 6.0.4

### Patch Changes

- e8973e8: Update swc_core to v10.2.3

## 6.0.3

### Patch Changes

- f436a09: Update swc_core to v10.

## 6.0.2

### Patch Changes

- f155bce: Update swc_core to v9

## 6.0.1

### Patch Changes

- c9e75ce: Bump crate versions

## 6.0.0

### Major Changes

- 4574a70: Update swc_core to v8.0.1

## 5.0.2

### Patch Changes

- f3cea5f: Bump versions

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
