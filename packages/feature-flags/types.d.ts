/**
 * SWC Feature Flags Plugin - TypeScript Definitions
 *
 * This plugin performs build-time transformation of feature flags.
 */

declare module "@swc/plugin-feature-flags" {
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
   * Build-time configuration for the feature flags plugin
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
 * ```json
 * {
 *   "jsc": {
 *     "experimental": {
 *       "plugins": [
 *         ["@swc/plugin-feature-flags", {
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
 */
