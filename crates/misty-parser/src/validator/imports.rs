use crate::validator::ValidationError;
use misty_ast::File;
use std::collections::HashMap;

/// Solves the imports of a module.
///
/// This is a simple algorithm where we check each import in the current module against the entire
/// workspace modules.
///
/// This function attests that at the very least, the module being imported is in scope.
pub fn attest_imports(
    imports: &[String],
    modules: &HashMap<String, File>,
) -> Result<(), ValidationError> {
    for import in imports {
        if !modules.contains_key(import) {
            tracing::trace!(?import, "Import not found in workspace");
            return Err(ValidationError::ImportedModuleNotFound);
        }
    }

    Ok(())
}
