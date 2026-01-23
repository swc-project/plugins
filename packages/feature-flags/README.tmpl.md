# @swc/plugin-experimental-feature-flags

SWC plugin for build-time feature flag transformation. Part of the SWC Feature Flags system.

## Overview

This plugin performs build-time marking of feature flags by replacing flag identifiers with `__SWC_FLAGS__` markers. This enables aggressive dead code elimination in subsequent build steps.

## Installation

```bash
npm install @swc/plugin-experimental-feature-flags
```

## Usage

Add to your `.swcrc`:

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
          }
        }]
      ]
    }
  }
}
```

## How It Works

**Before:**
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

**After:**
```javascript
function App() {
  if (__SWC_FLAGS__.featureA) {
    console.log('Feature A enabled');
  }

  return __SWC_FLAGS__.featureB ? 'Beta' : 'Stable';
}
```

The plugin:
1. Removes import statements from configured libraries
2. Detects destructuring patterns from configured functions
3. Replaces all flag identifier references with `__SWC_FLAGS__.flagName`
4. Removes the hook call statements

## Configuration

```typescript
interface BuildTimeConfig {
  /**
   * Library configurations: library name -> config
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
   * Flags to exclude from build-time marking
   *
   * These flags will not be transformed and will remain as-is.
   * Useful for flags that don't need dead code elimination.
   *
   * @default []
   */
  excludeFlags?: string[];

  /**
   * Global object name for markers
   *
   * @default "__SWC_FLAGS__"
   */
  markerObject?: string;
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

## Excluding Flags

You can exclude specific flags from transformation:

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
          "excludeFlags": ["quickToggle", "tempDebugFlag"]
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

## Scope Safety

The plugin correctly handles variable shadowing using SWC's syntax context system:

```javascript
import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA } = useExperimentalFlags();

  if (featureA) {  // Replaced with __SWC_FLAGS__.featureA
    const featureA = false;  // Shadowed variable
    if (featureA) {  // NOT replaced - uses local variable
      console.log('This uses the local featureA');
    }
  }
}
```

## Next Steps: Runtime Dead Code Elimination

After build-time transformation, use the `swc_feature_flags` Rust crate to:
1. Substitute `__SWC_FLAGS__.flagName` with actual boolean values
2. Eliminate dead code branches (if statements, ternary, logical operators)
3. Track statistics (bytes removed, branches eliminated)

See the [`swc_feature_flags` crate documentation](../../crates/swc_feature_flags/README.md) for more details.

## TypeScript Support

This package includes TypeScript definitions. See `types.d.ts` for the full API.

# CHANGELOG

$CHANGELOG

## License

Apache-2.0

## Links

- [GitHub Repository](https://github.com/swc-project/plugins)
- [SWC Website](https://swc.rs)
- [Rust Crate Documentation](../../crates/swc_feature_flags/README.md)
