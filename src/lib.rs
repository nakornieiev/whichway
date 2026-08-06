//! Read-only diagnostics for shell command resolution.
//!
//! `whichway` inspects a `PATH` variable and explains how a command name resolves
//! to executables on disk — including shadowed duplicates, symlinks, shims, and
//! version-manager-owned binaries.
//!
//! # Example
//!
//! ```
//! use whichway::resolvers::resolve_all;
//!
//! let path = std::env::var("PATH").unwrap_or_default();
//! let matches = resolve_all("python", &path);
//!
//! if let Some(active) = matches.first() {
//!     assert!(active.is_active);
//! }
//! ```

/// PATH-wide health checks: duplicates, broken symlinks, orphaned shims.
pub mod doctor;
/// Human-readable formatting for CLI output.
pub mod report;
/// Command resolution: scan `PATH`, classify executables, build match chains.
pub mod resolvers;
/// Heuristics for identifying version managers from path or shim content.
pub mod shim_detect;
