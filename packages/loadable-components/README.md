# loadable-components

## Config

```json
["loadable-components", {}]
```

Sometimes you need to wrap loadable with your own custom logic. There are many use cases for it, from injecting telemetry to hiding external libraries behind facade.
By default `loadable-components` are configured to transform dynamic imports used only inside loadable helpers, but can be configured to instrument any other function of your choice.
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

# @swc/plugin-loadable-components

## 2.0.1

### Patch Changes

- 04548e2: Update swc_core to 0.103.x

## 2.0.0

### Major Changes

- f8e5fd0: Update swc_core to 0.102.x

## 1.0.12

### Patch Changes

- 7d17e25: Update swc_core to v0.101.x

## 1.0.11

### Patch Changes

- 7391419: Update swc_core to v0.100.0

## 1.0.10

### Patch Changes

- 9c28afb: Update swc_core to 0.99.x (@swc/core 1.7.0)

## 1.0.9

### Patch Changes

- 979274e: Transpile lazy and signatures configuration

## 1.0.8

### Patch Changes

- af25741: Update swc_core to 0.96.0

## 1.0.7

### Patch Changes

- 41a8f56: Update swc_core to v0.95.x

## 1.0.6

### Patch Changes

- fc30490: Update swc_core to v0.93.0

## 1.0.5

### Patch Changes

- a7491af: Support single quote

## 1.0.4

### Patch Changes

- 0f38844: Publish all chanages

## 1.0.3

### Patch Changes

- 1cc9eda: Update dependencies

## 1.0.2

### Patch Changes

- 247cca6: Update rustc to 'nightly-2024-04-16'

## 1.0.1

### Patch Changes

- 876bbce: Update swc_core to 0.92.x

## 1.0.0

### Major Changes

- 8e91d39: Update swc_core to 0.91.x

## 0.3.121

### Patch Changes

- f4df366: Update swc_core

## 0.3.120

### Patch Changes

- c88b22b: Align package metadata

## 0.3.119

### Patch Changes

- a3cc4fb: Organize pacakge metadata

## 0.3.118

### Patch Changes

- e9e78ef: Update swc crates

## 0.3.117

### Patch Changes

- 6096d6d: Fix plugin version schema issue

## 0.3.116

### Patch Changes

- 37d3aaf: Depend on the swc download counter package

## 0.3.115

### Patch Changes

- 8bd92c7: swc_core 0.90.x

## 0.3.114

### Patch Changes

- 4ef0b7f: Add changelog to the readme

## 0.3.113

### Patch Changes

- 4e72680: swc_core@0.88.0

## 0.3.112

### Patch Changes

- 16bb4d8: swc_core@0.82.x
