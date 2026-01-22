use crate::Definition;

/// Root node of a Misty source file.
#[derive(Clone)]
pub struct File {
    /// List of imported modules.
    pub imports: Vec<String>,

    /// List of definitions in the file.
    pub definitions: Vec<Definition>,
}
