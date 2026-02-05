# SWC Feature Flags

A two-phase feature flag system for SWC that provides build-time marking and runtime dead code elimination.

## Overview

This library enables powerful feature flag management with aggressive dead code elimination. It works in two phases:

1. **Build-time (SWC Plugin)**: Marks feature flag usage locations by replacing flag identifiers with `__SWC_FLAGS__` markers
2. **Runtime (Standalone Crate)**: Substitutes flag values and eliminates dead code branches

## Features

- ✅ **Multiple usage patterns**: Direct destructuring, indirect destructuring, and property access
- ✅ **Customizable function names**: Not hardcoded to specific function names
- ✅ **Scope-safe**: Uses SWC's `Id` system to handle variable shadowing correctly
- ✅ **Dead code elimination**: Removes unreachable code branches
- ✅ **Statistics tracking**: Reports bytes removed and branches eliminated
- ✅ **Minifier-safe markers**: Uses `__SWC_FLAGS__` pattern that minifiers preserve

## Architecture

### Phase 1: Build-Time Transformation

The build-time plugin (`@swc/plugin-feature-flags`) performs these transformations:

**Input:**
```javascript
import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA, featureB } = useExperimentalFlags();

  if (featureA) {
    console.log('Feature A enabled');
  }

  return featureB ? 'Beta' : 'Stable';
}
```

**Output:**
```javascript
function App() {
  if (__SWC_FLAGS__.featureA) {
    console.log('Feature A enabled');
  }

  return __SWC_FLAGS__.featureB ? 'Beta' : 'Stable';
}
```

The plugin:
1. Tracks imports from configured libraries
2. Detects destructuring from configured flag functions
3. Replaces flag identifiers with `__SWC_FLAGS__.flagName` markers
4. Removes import statements and hook calls

### Supported Usage Patterns

The build-time plugin supports multiple ways of accessing feature flags:

#### Pattern 1: Direct Destructuring
```javascript
import { useExperimentalFlags } from '@their/library';

const { featureA, featureB } = useExperimentalFlags();
if (featureA) {
  // Transformed to: if (__SWC_FLAGS__.featureA)
}
```

#### Pattern 2: Indirect Destructuring
```javascript
import { useExperimentalFlags } from '@their/library';

const flags = useExperimentalFlags();
const { featureA } = flags;
if (featureA) {
  // Transformed to: if (__SWC_FLAGS__.featureA)
}
```

#### Pattern 3: Property Access
```javascript
import { useExperimentalFlags } from '@their/library';

const flags = useExperimentalFlags();
if (flags.featureA) {
  // Transformed to: if (__SWC_FLAGS__.featureA)
}
```

All three patterns are transformed identically and can be mixed in the same file.

### Phase 2: Runtime Transformation

The runtime transformer substitutes flag values and eliminates dead code:

**Input (from Phase 1):**
```javascript
function App() {
  if (__SWC_FLAGS__.featureA) {
    console.log('Feature A enabled');
  }

  return __SWC_FLAGS__.featureB ? 'Beta' : 'Stable';
}
```

**Runtime Config:**
```json
{
  "featureA": true,
  "featureB": false
}
```

**Output:**
```javascript
function App() {
  console.log('Feature A enabled');

  return 'Stable';
}
```

## Installation

### Rust API

Add to your `Cargo.toml`:

```toml
[dependencies]
swc_feature_flags = "0.1"
```

### SWC Plugin (WASM)

```bash
npm install @swc/plugin-feature-flags
```

## Usage

### Rust API

```rust
use swc_feature_flags::{build_time_pass, runtime_pass, BuildTimeConfig, RuntimeConfig, LibraryConfig};
use std::collections::HashMap;
use swc_ecma_transforms_base::resolver;
use swc_common::Mark;

// Build-time configuration
let mut libraries = HashMap::new();
libraries.insert(
    "@their/library".to_string(),
    LibraryConfig {
        functions: vec!["useExperimentalFlags".to_string()],
    },
);

let build_config = BuildTimeConfig {
    libraries,
    marker_object: "__SWC_FLAGS__".to_string(),
};

// Apply resolver first (required for scope safety)
let unresolved_mark = Mark::new();
let top_level_mark = Mark::new();
program = program.apply(resolver(unresolved_mark, top_level_mark, false));

// Apply build-time pass
program = program.apply(build_time_pass(build_config));

// Runtime configuration
let mut flag_values = HashMap::new();
flag_values.insert("featureA".to_string(), true);
flag_values.insert("featureB".to_string(), false);

let runtime_config = RuntimeConfig {
    flag_values,
    remove_markers: true,
    collect_stats: true,
    marker_object: "__SWC_FLAGS__".to_string(),
};

// Apply runtime pass
program = program.apply(runtime_pass(runtime_config));
```

### SWC Plugin (.swcrc)

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@swc/plugin-feature-flags", {
          "libraries": {
            "@their/library": {
              "functions": ["useExperimentalFlags", "getExperimentalFlags"]
            },
            "@another/flags": {
              "functions": ["useFeatures"]
            }
          },
          "markerObject": "__SWC_FLAGS__"
        }]
      ]
    }
  }
}
```

## Configuration

### Build-Time Config

```typescript
interface BuildTimeConfig {
  /** Library configurations: library name -> config */
  libraries: Record<string, LibraryConfig>;

  /** Global object name for markers (default: "__SWC_FLAGS__") */
  markerObject?: string;
}

interface LibraryConfig {
  /** Function names to detect (e.g., ["useExperimentalFlags"]) */
  functions: string[];
}
```

### Runtime Config

```rust
pub struct RuntimeConfig {
    /// Flag values to apply (flag_name -> boolean)
    pub flag_values: HashMap<String, bool>,

    /// Whether to remove markers after processing
    pub remove_markers: bool, // default: true

    /// Whether to collect statistics
    pub collect_stats: bool, // default: true

    /// Marker object name (must match build-time)
    pub marker_object: String, // default: "__SWC_FLAGS__"
}
```

## Dead Code Elimination

The runtime transformer eliminates:

### If Statements
```javascript
// Input
if (__SWC_FLAGS__.featureA) {  // true
  console.log('A');
} else {
  console.log('B');
}

// Output
console.log('A');
```

### Ternary Expressions
```javascript
// Input
const result = __SWC_FLAGS__.featureB ? 'On' : 'Off';  // false

// Output
const result = 'Off';
```

### Logical Operators
```javascript
// Input
const a = __SWC_FLAGS__.featureA && expensive();  // true
const b = __SWC_FLAGS__.featureB && shouldNotRun();  // false
const c = __SWC_FLAGS__.featureA || fallback();  // true

// Output
const a = expensive();
const b = false;
const c = true;
```

### Negation
```javascript
// Input
const notA = !__SWC_FLAGS__.featureA;  // true

// Output
const notA = false;
```

## Scope Safety

The library uses SWC's `Id` system (symbol + syntax context) to handle variable shadowing correctly:

```javascript
import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA } = useExperimentalFlags();

  if (featureA) {  // Replaced with __SWC_FLAGS__.featureA
    console.log('Outer');

    const featureA = false;  // Shadowed variable
    if (featureA) {  // NOT replaced - uses local variable
      console.log('Inner');
    }
  }
}
```

**Output:**
```javascript
function App() {
  if (__SWC_FLAGS__.featureA) {
    console.log('Outer');

    const featureA = false;
    if (featureA) {
      console.log('Inner');
    }
  }
}
```

## Statistics

When `collect_stats` is enabled, the runtime transformer tracks:

```rust
pub struct TransformStats {
    pub original_bytes: usize,      // Approximate original size
    pub removed_bytes: usize,        // Bytes removed by DCE
    pub branches_eliminated: usize,  // Number of branches eliminated
    pub flags_processed: HashSet<String>, // Flags that were processed
}
```

## Testing

```bash
# Run all tests
cargo test -p swc_feature_flags

# Run fixture tests only
cargo test -p swc_feature_flags --test fixture
```

## License

Apache-2.0

## Contributing

Contributions are welcome! Please ensure:
- All tests pass
- Code follows Rust formatting guidelines
- New features include tests
