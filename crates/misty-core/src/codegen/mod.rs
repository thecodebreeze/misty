use misty_parser::Workspace;
use std::error::Error as StdError;
use std::path::Path;

/// Main trait to define a common interface for all code generators supported by Misty.
pub trait CodeGenerator {
    /// Error type returned by the code generator.
    type Error: StdError + Send + Sync + 'static;

    /// Options required by the code generator.
    type Options;

    /// Generates the code for the given workspace.
    ///
    /// This function outputs all generated code in the specified folder according to the
    /// generator's own logic.
    fn generate(
        &self,
        options: &Self::Options,
        workspace: &Workspace,
        output_dir: &Path,
    ) -> Result<(), Self::Error>;
}
