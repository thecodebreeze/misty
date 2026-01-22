use crate::validator::ValidationError;
use misty_ast::{DataType, Definition, File};
use std::collections::HashMap;

/// Resolve a [DataType] against the workspace.
///
/// This function will check both containers and user-defined types by executing a module-local
/// lookup first, then a workspace lookup.
///
/// Primitives are not resolved because they are guaranteed to be present in the workspace.
///
/// Failing fast at any first resolution failure it founds.
pub fn resolve_data_type(
    modules: &HashMap<String, File>,
    file: &File,
    data_type: &DataType,
) -> Result<(), ValidationError> {
    match data_type {
        // Skipped since they are always valid.
        DataType::Primitive(_) => (),
        // We treat this recursively since containers can contain other containers.
        DataType::Container(_, inner) => resolve_data_type(modules, file, inner)?,
        // Look up the type in the both the local and workspace modules.
        DataType::UserType(type_name) => lookup_type(modules, file, type_name)?,
    }

    Ok(())
}

/// Helper method to look up a type in the workspace and the current module file.
fn lookup_type(
    modules: &HashMap<String, File>,
    file: &File,
    type_name: &str,
) -> Result<(), ValidationError> {
    // If the type name contains a dot, this means this is either a `package-local` or
    // `package-remote` type.
    //
    // Otherwise, it's a `package-local` type, and we resolve it against the same file.
    if type_name.contains('.') {
        // The last part of the name is the actual type name, the rest is the module path.
        let (import_path, type_name) = match type_name.rsplit_once('.') {
            Some(parts) => parts,
            None => {
                tracing::debug!(?type_name, "Invalid type name");
                return Err(ValidationError::ModuleImportPath);
            }
        };

        // Check if the file is importing the import path of the module.
        if !file.imports.contains(&import_path.to_string()) {
            tracing::debug!(?import_path, ?type_name, "Module import not found");
            return Err(ValidationError::ModuleNotImported);
        }

        // Get the file from the workspace which, in theory, contains the required type.
        //
        // We handle the failing use case instead of unwrapping, even though unwrapping here would
        // actually be safe since we validate all imports beforehand.
        let type_module_file = match modules.get(import_path) {
            Some(file) => file,
            None => {
                tracing::debug!(?import_path, ?type_name, "Module not found in workspace");
                return Err(ValidationError::ImportedModuleNotFound);
            }
        };

        // Check if the type actually exists in the target module file.
        if !file_has_type(type_module_file, type_name) {
            tracing::debug!(?import_path, ?type_name, "Type not found in module");
            return Err(ValidationError::TypeNotFound);
        }

        Ok(())
    } else {
        if !file_has_type(file, type_name) {
            tracing::debug!(?type_name, "Type not found in file");
            return Err(ValidationError::TypeNotFound);
        }

        Ok(())
    }
}

/// Helper method to check if a file contains a definition with a given type name.
fn file_has_type(file: &File, type_name: &str) -> bool {
    file.definitions.iter().any(|definition| match definition {
        Definition::Interface(interface) => interface.name.eq(type_name),
        Definition::Schema(schema) => schema.name.eq(type_name),
        Definition::Enum(misty_enum) => misty_enum.name.eq(type_name),
    })
}
