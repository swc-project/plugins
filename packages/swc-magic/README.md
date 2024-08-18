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

### `markAsPure`

```js
import { markAsPure } from "@swc/magic";

markAsPure(() => console.log("This will be removed by the SWC minifier"));
```

# @swc/plugin-swc-magic

## 2.0.10

### Patch Changes

- 7d17e25: Update swc_core to v0.101.x

## 2.0.9

### Patch Changes

- 7391419: Update swc_core to v0.100.0

## 2.0.8

### Patch Changes

- 9c28afb: Update swc_core to 0.99.x (@swc/core 1.7.0)

## 2.0.7

### Patch Changes

- af25741: Update swc_core to 0.96.0

## 2.0.6

### Patch Changes

- 41a8f56: Update swc_core to v0.95.x

## 2.0.5

### Patch Changes

- fc30490: Update swc_core to v0.93.0

## 2.0.4

### Patch Changes

- 0f38844: Publish all chanages

## 2.0.3

### Patch Changes

- 1cc9eda: Update dependencies

## 2.0.2

### Patch Changes

- 247cca6: Update rustc to 'nightly-2024-04-16'

## 2.0.1

### Patch Changes

- 876bbce: Update swc_core to 0.92.x

## 2.0.0

### Major Changes

- 8e91d39: Update swc_core to 0.91.x

## 1.5.121

### Patch Changes

- f4df366: Update swc_core

## 1.5.120

### Patch Changes

- c88b22b: Align package metadata

## 1.5.119

### Patch Changes

- a3cc4fb: Organize pacakge metadata

## 1.5.118

### Patch Changes

- e9e78ef: Update swc crates

## 1.5.117

### Patch Changes

- 6096d6d: Fix plugin version schema issue

## 1.5.116

### Patch Changes

- 37d3aaf: Depend on the swc download counter package

## 1.5.115

### Patch Changes

- 8bd92c7: swc_core 0.90.x

## 1.5.114

### Patch Changes

- 4ef0b7f: Add changelog to the readme

## 1.5.113

### Patch Changes

- 4e72680: swc_core@0.88.0

## 1.5.112

### Patch Changes

- 16bb4d8: swc_core@0.82.x
