mod parser;

pub use parser::ParserError;

/// General purpose error type for Misty.
///
/// This error type encompasses all error types across Misty crates.
#[derive(Debug, thiserror::Error)]
pub enum MistyError {
    #[error("Parser Error: {0}")]
    Parser(#[from] ParserError),
}
