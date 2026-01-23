use crate::Status;

/// Enum that defines all possible errors that can occur during network operations.
#[derive(Debug, thiserror::Error)]
pub enum NetError {
    #[error("Internal IO error: {0}")]
    StdIo(#[from] std::io::Error),

    #[error("Procedure failed with status: {0:?}")]
    Status(Status),
}
