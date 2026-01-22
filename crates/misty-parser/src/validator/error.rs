/// Validation specific errors.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("An imported module was not found in the workspace")]
    ImportedModuleNotFound,

    #[error("The module import path is invalid")]
    ModuleImportPath,

    #[error("A required module was not imported")]
    ModuleNotImported,

    #[error("A required imported type was not found in the target module")]
    TypeNotFound,
}
