### `@swc/plugin-styled-components`

#### Setup

```sh
npm install --save-dev @swc/plugin-styled-components @swc/core
```

Then update your `.swcrc` file like below:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@swc/plugin-styled-components",
          {
            "displayName": true,
            "ssr": true
          }
        ]
      ]
    }
  }
}
```

# @swc/plugin-styled-components

## 11.0.0

### Major Changes

- 593f438: Update swc_core to v47

## 10.0.0

### Major Changes

- 25e0c2c: Update swc_core to v46.0.0

## 9.5.0

### Minor Changes

- c324a59: Update swc_core to v45

## 9.4.0

### Minor Changes

- 8bad98d: Update swc_core to v44

## 9.3.0

### Minor Changes

- 47be132: Update swc_core to v42

## 9.2.0

### Minor Changes

- 0c9d7a1: build: Update swc_core to v39.0.0

## 9.1.0

### Minor Changes

- a872100: build: Update swc_core to v38

## 9.0.4

### Patch Changes

- 9cdcdc5: Update swc_core to v36

## 9.0.3

### Patch Changes

- 9b08ff7: Update swc_core to v35

## 9.0.2

### Patch Changes

- 27cae40: Disallow typescript types to shrink size

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

## 7.1.6

### Patch Changes

- e3e743d: Update swc_core to v27

## 7.1.5

### Patch Changes

- 0d14c35: Use correct span hygiene

## 7.1.4

### Patch Changes

- 5ddbaeb: Update swc_core to v23

## 7.1.3

### Patch Changes

- d51d525: Update swc_core to v22.0.0

## 7.1.2

### Patch Changes

- cb94b92: Update swc_core to v21.0.1

## 7.1.1

### Patch Changes

- 31e3254: build: Update `swc_core` to `v19.0.0`

## 7.1.0

### Minor Changes

- bb6ba9b: Matches babel plugin in naming

## 7.0.0

### Major Changes

- f0fee1d: Update swc_core to v15.0.1

## 6.8.2

### Patch Changes

- 04465bc: Update swc_core to v14.0.0, really

## 6.8.1

### Patch Changes

- ce8d317: Update swc_core to v14.0.0

## 6.8.0

### Minor Changes

- bfa0a51: Update swc_core to v13

## 6.7.0

### Minor Changes

- b8c4e6c: Update swc_core to v12

## 6.6.0

### Minor Changes

- 4c8b0e2: Update swc_core

## 6.5.0

### Minor Changes

- 04c2925: Improve performance

## 6.4.0

### Minor Changes

- 1a6bbf1: Improve performance

## 6.3.0

### Minor Changes

- 4e7336c: Remove needless allocations

## 6.2.0

### Minor Changes

- 54b4a1a: Remove needless allocations

## 6.1.0

### Minor Changes

- d532813: Avoid allocation when not used

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

## 2.0.12

### Patch Changes

- 7d17e25: Update swc_core to v0.101.x

## 2.0.11

### Patch Changes

- 7391419: Update swc_core to v0.100.0

## 2.0.10

### Patch Changes

- 9c28afb: Update swc_core to 0.99.x (@swc/core 1.7.0)

## 2.0.9

### Patch Changes

- af25741: Update swc_core to 0.96.0

## 2.0.8

### Patch Changes

- 41a8f56: Update swc_core to v0.95.x

## 2.0.7

### Patch Changes

- fc30490: Update swc_core to v0.93.0

## 2.0.6

### Patch Changes

- 0f38844: Publish all chanages

## 2.0.5

### Patch Changes

- 1cc9eda: Update dependencies

## 2.0.4

### Patch Changes

- 247cca6: Update rustc to 'nightly-2024-04-16'

## 2.0.3

### Patch Changes

- 876bbce: Update swc_core to 0.92.x

## 2.0.2

### Patch Changes

- 7f8ff1b: Use template literals

## 2.0.1

### Patch Changes

- 71a0922: Fix escape

## 2.0.0

### Major Changes

- 8e91d39: Update swc_core to 0.91.x

## 1.5.122

### Patch Changes

- f4df366: Update swc_core

## 1.5.121

### Patch Changes

- c88b22b: Align package metadata

## 1.5.120

### Patch Changes

- a3cc4fb: Organize pacakge metadata

## 1.5.119

### Patch Changes

- e9e78ef: Update swc crates

## 1.5.118

### Patch Changes

- 6096d6d: Fix plugin version schema issue

## 1.5.117

### Patch Changes

- 37d3aaf: Depend on the swc download counter package

## 1.5.116

### Patch Changes

- 8bd92c7: swc_core 0.90.x

## 1.5.115

### Patch Changes

- 34f9d21: Fix escaping issue

## 1.5.114

### Patch Changes

- 4ef0b7f: Add changelog to the readme

## 1.5.113

### Patch Changes

- 4e72680: swc_core@0.88.0

## 1.5.112

### Patch Changes

- 16bb4d8: swc_core@0.82.x
