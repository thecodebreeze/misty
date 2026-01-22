//! The validator module contains code to build Misty workspaces for validation purposes.
//!
//! Validating starts by adding all detected modules, either `package-local` or `package-remote`
//! into the workspace. Then, we check each file if their imports are to be found in the modules'
//! registry. Afterward, all definitions are validated by resolving their `module-local`,
//! `package-local`, and `package-remote` user-defined types.
//!
//! `package-remote` is currently not implemented, it is planned for implementation with the
//! package manager and registry feature.
mod error;
mod imports;
mod type_resolver;
mod workspace;

pub use error::ValidationError;
pub use workspace::Workspace;
