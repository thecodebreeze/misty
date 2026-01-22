//! The Workspace module contains the logic to build a Misty workspace for validation purposes.
//!
//! The validation method uses a fail-fast strategy instead of returning all cumulative errors, this
//! is simply to reduce complexity. It's not a hard requirement and can be improved later.
//!
//! As `package-remote` modules are not yet here, we only validate against `package-local` modules.
use crate::ParserError;
use crate::validator::ValidationError;
use crate::validator::imports::attest_imports;
use crate::validator::type_resolver::resolve_data_type;
use misty_ast::{Definition, File};
use std::collections::HashMap;

/// The project workspace.
///
/// It contains all modules imported and validates them against each other.
pub struct Workspace {
    /// Modules found in the current workspace.
    package_local_modules: HashMap<String, File>,

    /// Flag that marks this Workspace as validated.
    validated: bool,
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

impl Workspace {
    /// Creates a new, empty workspace.
    pub fn new() -> Self {
        Self {
            package_local_modules: HashMap::new(),
            validated: false,
        }
    }

    /// Adds a new local module to the workspace.
    pub fn add_local_module(&mut self, module_path: &str, module: File) {
        self.package_local_modules
            .insert(module_path.to_string(), module);
    }

    /// Validates all modules in the workspace.
    ///
    /// This function is fail-fast. Meaning, it will fail at each first error it founds.
    #[tracing::instrument(skip(self))]
    pub fn validate(&mut self) -> Result<(), ParserError> {
        for file in self.package_local_modules.values() {
            self.validate_file(file)?;
        }

        self.validated = true;
        Ok(())
    }

    /// Validates a single file.
    ///
    /// This function is fail-fast. It will return the first error it finds as soon as it does.
    #[tracing::instrument(skip(self, file))]
    fn validate_file(&self, file: &File) -> Result<(), ValidationError> {
        // Run a fast check in the imports of the file to attest that at the very least, the module
        // is in scope.
        attest_imports(&file.imports, &self.package_local_modules)?;

        // Validate all the definitions in the file.
        for definition in &file.definitions {
            match definition {
                // Enums do not reference a type, so they are skipped from validation.
                Definition::Enum(_) => (),
                Definition::Interface(interface) => {
                    for function in &interface.functions {
                        // Resolve the input argument type.
                        resolve_data_type(&self.package_local_modules, file, &function.input.1)?;

                        // Resolve the output argument type.
                        if let Some((_, argument)) = &function.output {
                            resolve_data_type(&self.package_local_modules, file, argument)?;
                        }
                    }
                }
                Definition::Schema(schema) => {
                    for field in &schema.fields {
                        resolve_data_type(&self.package_local_modules, file, &field.field_type)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Gets a reference to the local modules in the workspace.
    pub fn package_local_modules(&self) -> &HashMap<String, File> {
        &self.package_local_modules
    }

    /// Checks if the workspace has been validated.
    pub fn validated(&self) -> bool {
        self.validated
    }
}
