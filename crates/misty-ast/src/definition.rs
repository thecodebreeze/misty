use crate::{Enum, Interface, Schema};

/// Definitions are components that compose a Misty file.
#[derive(Clone)]
pub enum Definition {
    /// Interfaces are the gateway to services.
    Interface(Interface),

    /// Schemas are the bodies/messages that are transmitted between clients and servers.
    Schema(Schema),

    /// Enums are similar to Protobuf's enums.
    ///
    /// Unlike the other definitions, Misty Enum's are very similar to C and TypeScript enums in the
    /// way they are laid out, they are represented as a Rust `u16`.
    Enum(Enum),
}
