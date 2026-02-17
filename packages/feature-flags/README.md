# @swc/plugin-experimental-feature-flags

SWC plugin for build-time feature flag transformation. Part of the SWC Feature Flags system.

## Overview

This plugin provides two modes for feature flag transformation:

- **Mark mode** (default): Marks flags with `__SWC_FLAGS__` markers for later substitution. Use this when you want to perform flag substitution in a separate build step.

- **Shake mode**: Directly substitutes flag values with boolean literals and performs dead code elimination in a single pass. Use this when you know the flag values at build time.

## Installation

```bash
npm install @swc/plugin-experimental-feature-flags
```

## Usage

### Mark Mode (Default - Marker Generation)

Use mark mode to mark flags for later substitution:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@swc/plugin-experimental-feature-flags", {
          "mode": "mark",
          "libraries": {
            "@their/library": {
              "functions": ["useExperimentalFlags"]
            }
          }
        }]
      ]
    }
  }
}
```

### Shake Mode (Direct Optimization with DCE)

Use shake mode to directly substitute flag values and eliminate dead code.
This mode operates on `__SWC_FLAGS__` markers and does not use `libraries`:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@swc/plugin-experimental-feature-flags", {
          "mode": "shake",
          "flagValues": {
            "featureA": true,
            "featureB": false
          }
        }]
      ]
    }
  }
}
```

## How It Works

### Mark Mode Example

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

The plugin in mark mode:
1. Removes import statements from configured libraries
2. Detects destructuring patterns from configured functions
3. Replaces all flag identifier references with `__SWC_FLAGS__.flagName`
4. Removes the hook call statements

### Shake Mode Example

**Input:** (same as above)

**Output (with `featureA: true, featureB: false`):**
```javascript
function App() {
  console.log('Feature A enabled');
  return 'Stable';
}
```

The plugin in shake mode:
1. Substitutes `__SWC_FLAGS__` markers with boolean literals
2. Performs dead code elimination (DCE)

## Configuration

```typescript
interface FeatureFlagsConfig {
  /**
   * Transformation mode
   *
   * - "mark" (default): Marker-based - replaces flags with __SWC_FLAGS__.flagName
   *   for later substitution
   * - "shake": Direct optimization - substitutes flags with boolean values
   *   and performs DCE immediately
   *
   * @default "mark"
   */
  mode?: "mark" | "shake";

  /**
   * Library configurations: library name -> config
   * Required in mark mode, not used in shake mode
   *
   * @example
   * {
   *   "@their/library": {
   *     functions: ["useExperimentalFlags"]
   *   },
   *   "@another/flags": {
   *     functions: ["useFeatures", "getFeatures"]
   *   }
   * }
   */
  libraries: Record<string, LibraryConfig>;

  /**
   * Global object name for markers
   * Only used in mark mode
   *
   * @default "__SWC_FLAGS__"
   */
  markerObject?: string;

  /**
   * Flag values to apply (flag_name -> boolean)
   * Required in shake mode, not used in mark mode
   *
   * @example
   * {
   *   "featureA": true,
   *   "featureB": false
   * }
   */
  flagValues?: Record<string, boolean>;

  /**
   * Whether to collect transformation statistics
   * Only used in shake mode
   *
   * @default true
   */
  collectStats?: boolean;
}

interface LibraryConfig {
  /**
   * Function names to detect for this library
   * @example ["useExperimentalFlags", "getExperimentalFlags"]
   */
  functions: string[];
}
```

## Multiple Libraries

You can configure multiple libraries:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@swc/plugin-experimental-feature-flags", {
          "libraries": {
            "@their/library": {
              "functions": ["useExperimentalFlags", "getExperimentalFlags"]
            },
            "@another/flags": {
              "functions": ["useFeatures"]
            },
            "@custom/flags": {
              "functions": ["getFlags", "useFlags"]
            }
          }
        }]
      ]
    }
  }
}
```

## Custom Marker Object

You can customize the marker object name:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@swc/plugin-experimental-feature-flags", {
          "libraries": {
            "@their/library": {
              "functions": ["useExperimentalFlags"]
            }
          },
          "markerObject": "__FEATURE_FLAGS__"
        }]
      ]
    }
  }
}
```

## Two-Phase Workflow

This plugin uses a **two-phase approach** for feature flag transformation:

1. **Phase 1 (Mark mode)**: Convert flag variables to `__SWC_FLAGS__` markers
2. **Phase 2 (Shake mode)**: Substitute markers with boolean values and eliminate dead code

### When to Use Each Mode

**Mark Mode (Phase 1)**
- Use in your initial build/compilation step
- Converts flag variables from your flag library into markers
- Output code still contains conditional logic (no DCE yet)
- Run this on your source code before bundling

**Shake Mode (Phase 2)**
- Use after mark mode, when you know flag values
- Substitutes `__SWC_FLAGS__` markers with actual boolean values
- Performs dead code elimination (DCE)
- Run this in your build pipeline with environment-specific flag values

### Example Workflow

```bash
# Phase 1: Mark flags in source code
# swc.config.json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@swc/plugin-experimental-feature-flags", {
          "mode": "mark",
          "libraries": {
            "@your/flags": { "functions": ["useFlags"] }
          }
        }]
      ]
    }
  }
}

# Phase 2: Shake (DCE) with environment-specific values
# swc.prod.config.json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@swc/plugin-experimental-feature-flags", {
          "mode": "shake",
          "flagValues": {
            "featureA": true,
            "featureB": false
          }
        }]
      ]
    }
  }
}
```

## Scope Safety

The plugin correctly handles variable shadowing using SWC's syntax context system:

```javascript
import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA } = useExperimentalFlags();

  if (featureA) {  // Replaced (mode dependent)
    const featureA = false;  // Shadowed variable
    if (featureA) {  // NOT replaced - uses local variable
      console.log('This uses the local featureA');
    }
  }
}
```

## Complete Two-Phase Example

Here's a complete workflow showing both phases:

**Step 1: Source Code**
```javascript
import { useFlags } from '@your/flags';

function App() {
  const { newUI, betaFeature } = useFlags();

  if (newUI) {
    return <NewDashboard />;
  }

  return betaFeature ? <BetaApp /> : <StableApp />;
}
```

**Step 2: After Mark Mode**
```javascript
function App() {
  if (__SWC_FLAGS__.newUI) {
    return <NewDashboard />;
  }

  return __SWC_FLAGS__.betaFeature ? <BetaApp /> : <StableApp />;
}
```

**Step 3: After Shake Mode (with `newUI: true, betaFeature: false`)**
```javascript
function App() {
  return <NewDashboard />;
}
```

All dead code has been eliminated!

## TypeScript Support

This package includes TypeScript definitions. See `types.d.ts` for the full API.

# ChangeLog

# @swc/plugin-experimental-feature-flags

## 0.5.0

### Minor Changes

- 2113ddb: build: Update swc_core to v57
- 207e42e: feat(swc_feature_flags): Support indirect destructuring and property access patterns

## 0.4.0

### Minor Changes

- 870774a: build: Update swc_core to v56

## 0.3.0

### Minor Changes

- 7c9c588: feat: Support both modes

## 0.2.0

### Minor Changes

- 66f5258: build: Update swc_core to v55.x.x

## License

Apache-2.0

## Links

- [GitHub Repository](https://github.com/swc-project/plugins)
- [SWC Website](https://swc.rs)
- [Rust Crate Documentation](../../crates/swc_feature_flags/README.md)
