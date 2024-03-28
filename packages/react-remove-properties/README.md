# react-remove-properties

See https://nextjs.org/docs/architecture/nextjs-compiler#remove-react-properties for more information.

## Config

```json
["react-remove-properties"]
```

or

```json
[
  "react-remove-properties",
  {
    // The regexes defined here are processed in Rust so the syntax is different from
    // JavaScript `RegExp`s. See https://docs.rs/regex.
    "properties": ["^data-custom$"]
  }
]
```

# @swc/plugin-react-remove-properties

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
