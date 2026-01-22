use misty_ast::File;
use std::collections::HashMap;

/// The project workspace.
///
/// It contains all modules imported and validates them against each other.
pub struct Workspace {
    /// Modules found in the current workspace.
    package_local_modules: HashMap<String, File>,
}
