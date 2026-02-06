/**
 * SWC Feature Flags Plugin - TypeScript Definitions
 *
 * This plugin performs build-time transformation of feature flags.
 */

declare module "@swc/plugin-experimental-feature-flags" {
  /**
   * Transform mode for feature flags
   */
  export type TransformMode = "mark" | "shake";

  /**
   * Configuration for a single library
   */
  export interface LibraryConfig {
    /**
     * Function names to detect for this library
     * @example ["useExperimentalFlags", "getFlags"]
     */
    functions: string[];
  }

  /**
   * Unified configuration for the feature flags plugin
   *
   * Supports two modes:
   * - **mark** (default): Marks flags with `__SWC_FLAGS__` markers for later
   *   substitution. Use this when you want to perform flag substitution
   *   in a separate build step.
   * - **shake**: Directly substitutes flag values with boolean literals and
   *   performs dead code elimination in a single pass. Use this for direct
   *   optimization when you know flag values at build time.
   *
   * @example Mark mode (marker generation)
   * ```json
   * {
   *   "mode": "mark",
   *   "libraries": {
   *     "@their/library": {
   *       "functions": ["useExperimentalFlags"]
   *     }
   *   }
   * }
   * ```
   *
   * @example Shake mode (direct optimization with DCE)
   * ```json
   * {
   *   "mode": "shake",
   *   "libraries": {
   *     "@their/library": {
   *       "functions": ["useExperimentalFlags"]
   *     }
   *   },
   *   "flagValues": {
   *     "featureA": true,
   *     "featureB": false
   *   }
   * }
   * ```
   */
  export interface FeatureFlagsConfig {
    /**
     * Transform mode
     *
     * - **mark** (default): Marker-based - replaces flags with `__SWC_FLAGS__.flagName`
     *   for later substitution
     * - **shake**: Direct optimization - substitutes flags with boolean values
     *   and performs DCE immediately
     *
     * @default "mark"
     */
    mode?: TransformMode;

    /**
     * Library configurations: library name -> config
     *
     * Required in mark mode, not used in shake mode.
     * The plugin will track imports from these libraries and
     * transform calls to the specified functions.
     *
     * @example
     * {
     *   "@their/library": {
     *     functions: ["useExperimentalFlags"]
     *   },
     *   "@another/flags": {
     *     functions: ["useFeatures"]
     *   }
     * }
     */
    libraries: Record<string, LibraryConfig>;

    /**
     * Global object name for markers
     *
     * Only used in mark mode. The plugin will replace flag identifiers
     * with member expressions on this global object.
     *
     * @default "__SWC_FLAGS__"
     */
    markerObject?: string;

    /**
     * Flag values to apply (flag_name -> boolean)
     *
     * Required in shake mode. Maps flag names to their boolean values.
     * The plugin will substitute these values directly and eliminate dead code.
     *
     * Not used in mark mode.
     *
     * @example
     * {
     *   "featureA": true,
     *   "featureB": false,
     *   "experimentalUI": true
     * }
     */
    flagValues?: Record<string, boolean>;

    /**
     * Whether to collect transformation statistics
     *
     * Only used in shake mode. When enabled, the plugin tracks how many
     * flags were processed and how much code was eliminated.
     *
     * @default true
     */
    collectStats?: boolean;
  }

  /**
   * Build-time configuration for the feature flags plugin
   *
   * @deprecated Use FeatureFlagsConfig with mode: "shake" instead
   */
  export interface BuildTimeConfig {
    /**
     * Library configurations: library name -> config
     *
     * The plugin will track imports from these libraries and
     * transform calls to the specified functions.
     *
     * @example
     * {
     *   "@their/library": {
     *     functions: ["useExperimentalFlags"]
     *   },
     *   "@another/flags": {
     *     functions: ["useFeatures"]
     *   }
     * }
     */
    libraries: Record<string, LibraryConfig>;

    /**
     * Global object name for markers
     *
     * The plugin will replace flag identifiers with member expressions
     * on this global object. This name should be unique and not conflict
     * with user code.
     *
     * @default "__SWC_FLAGS__"
     */
    markerObject?: string;
  }
}

/**
 * Example usage in .swcrc:
 *
 * Mark mode (marker generation for later substitution):
 * ```json
 * {
 *   "jsc": {
 *     "experimental": {
 *       "plugins": [
 *         ["@swc/plugin-experimental-feature-flags", {
 *           "mode": "mark",
 *           "libraries": {
 *             "@their/library": {
 *               "functions": ["useExperimentalFlags"]
 *             }
 *           }
 *         }]
 *       ]
 *     }
 *   }
 * }
 * ```
 *
 * Shake mode (direct optimization with DCE):
 * ```json
 * {
 *   "jsc": {
 *     "experimental": {
 *       "plugins": [
 *         ["@swc/plugin-experimental-feature-flags", {
 *           "mode": "shake",
 *           "libraries": {
 *             "@their/library": {
 *               "functions": ["useExperimentalFlags"]
 *             }
 *           },
 *           "flagValues": {
 *             "featureA": true,
 *             "featureB": false
 *           }
 *         }]
 *       ]
 *     }
 *   }
 * }
 * ```
 */
