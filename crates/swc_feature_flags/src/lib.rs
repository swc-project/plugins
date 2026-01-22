//! # SWC Feature Flags
//!
//! A two-phase feature flag system for SWC:
//! 1. **Build-time**: Mark feature flag usage with `__SWC_FLAGS__` markers
//! 2. **Runtime**: Substitute flag values and eliminate dead code
//!
//! ## Example
//!
//! ```rust,ignore
//! use swc_feature_flags::{build_time_pass, runtime_pass, BuildTimeConfig, RuntimeConfig};
//! use std::collections::HashMap;
//!
//! // Build-time configuration
//! let mut libraries = HashMap::new();
//! libraries.insert(
//!     "@their/library".to_string(),
//!     LibraryConfig {
//!         functions: vec!["useExperimentalFlags".to_string()],
//!     },
//! );
//!
//! let build_config = BuildTimeConfig {
//!     libraries,
//!     exclude_flags: vec![],
//!     marker_object: "__SWC_FLAGS__".to_string(),
//! };
//!
//! // Apply build-time pass
//! let program = program.apply(build_time_pass(build_config));
//!
//! // Runtime configuration
//! let mut flag_values = HashMap::new();
//! flag_values.insert("featureA".to_string(), true);
//! flag_values.insert("featureB".to_string(), false);
//!
//! let runtime_config = RuntimeConfig {
//!     flag_values,
//!     remove_markers: true,
//!     collect_stats: true,
//!     marker_object: "__SWC_FLAGS__".to_string(),
//! };
//!
//! // Apply runtime pass
//! let program = program.apply(runtime_pass(runtime_config));
//! ```

pub mod build_time;
pub mod config;
pub mod runtime;
pub mod stats;

// Re-exports for convenience
pub use build_time::BuildTimeTransform;
pub use config::{BuildTimeConfig, LibraryConfig, RuntimeConfig};
pub use runtime::RuntimeTransform;
pub use stats::TransformStats;
use swc_ecma_ast::Pass;
use swc_ecma_visit::visit_mut_pass;

/// Create a build-time pass that marks feature flags with `__SWC_FLAGS__`
/// markers
///
/// This pass will:
/// - Track imports from configured libraries
/// - Detect destructuring from configured flag functions
/// - Replace flag identifiers with `__SWC_FLAGS__.flagName`
/// - Remove import statements and hook calls
///
/// # Example
///
/// ```rust,ignore
/// let config = BuildTimeConfig {
///     libraries: HashMap::from([
///         ("@their/library".to_string(), LibraryConfig {
///             functions: vec!["useExperimentalFlags".to_string()],
///         }),
///     ]),
///     exclude_flags: vec![],
///     marker_object: "__SWC_FLAGS__".to_string(),
/// };
///
/// let program = program.apply(build_time_pass(config));
/// ```
pub fn build_time_pass(config: BuildTimeConfig) -> impl Pass {
    visit_mut_pass(BuildTimeTransform::new(config))
}

/// Create a runtime pass that substitutes flag values and eliminates dead code
///
/// This pass will:
/// - Locate `__SWC_FLAGS__.flagName` member expressions
/// - Replace with literal boolean values
/// - Eliminate dead code branches (if statements, ternary, logical operators)
/// - Track statistics
///
/// # Example
///
/// ```rust,ignore
/// let config = RuntimeConfig {
///     flag_values: HashMap::from([
///         ("featureA".to_string(), true),
///         ("featureB".to_string(), false),
///     ]),
///     remove_markers: true,
///     collect_stats: true,
///     marker_object: "__SWC_FLAGS__".to_string(),
/// };
///
/// let program = program.apply(runtime_pass(config));
/// ```
pub fn runtime_pass(config: RuntimeConfig) -> impl Pass {
    visit_mut_pass(RuntimeTransform::new(config))
}
