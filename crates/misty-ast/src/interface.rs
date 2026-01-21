use crate::Function;

/// Interfaces are similar to Protobuf's services.
///
/// They are composed of functions that can be streamable on both ends! Services allow functions
/// to be:
///
/// * Client Streaming
/// * Server Streaming
/// * Bidirectional Streaming
/// * Unary
///
/// And `Unary` calls can be `Muted` meaning that the server does not need to send data or
/// confirmations back to the client. Basically a fire-and-forget call.
#[derive(Clone)]
pub struct Interface {
    /// The name of the interface.
    ///
    /// Must be in PascalCase.
    pub name: String,

    /// The functions that compose the interface.
    pub functions: Vec<Function>,
}
