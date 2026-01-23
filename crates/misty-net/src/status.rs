use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum Status {
    Success = 0,
    ProtocolFailure = 1,
    HandshakeViolation = 2,
    MalformedFrame = 3,
    ServiceNotFound = 4,
    MethodNotFound = 5,
    InvalidDataFrame = 6,
    InternalServerError = 7,
    Custom(u16),
}

impl Status {
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::Success => 0,
            Self::ProtocolFailure => 1,
            Self::MalformedFrame => 2,
            Self::HandshakeViolation => 3,
            Self::ServiceNotFound => 4,
            Self::MethodNotFound => 5,
            Self::InvalidDataFrame => 6,
            Self::InternalServerError => 7,
            Self::Custom(code) => *code,
        }
    }
}

impl From<Status> for u16 {
    fn from(status: Status) -> Self {
        status.as_u16()
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::ProtocolFailure => write!(f, "PROTOCOL_FAILURE"),
            Self::MalformedFrame => write!(f, "MALFORMED_FRAME"),
            Self::HandshakeViolation => write!(f, "HANDSHAKE_VIOLATION"),
            Self::ServiceNotFound => write!(f, "SERVICE_NOT_FOUND"),
            Self::MethodNotFound => write!(f, "METHOD_NOT_FOUND"),
            Self::InvalidDataFrame => write!(f, "INVALID_DATA_FRAME"),
            Self::InternalServerError => write!(f, "INTERNAL_SERVER_ERROR"),
            Self::Custom(code) => write!(f, "CUSTOM({})", code),
        }
    }
}
