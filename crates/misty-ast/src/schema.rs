use crate::Field;

/// Schemas are very similar to Protobuf's messages.
///
/// They are composed of Fields formed from one of the available Data Types. They are very
/// simple and are very similar to Rust's structs, there's no need to number fields like it's
/// done in Protobuf; we use deterministic serialization and strict discriminator logic that
/// changes when a signature changes.
#[derive(Clone)]
pub struct Schema {
    /// The name of the schema.
    ///
    /// Must be in PascalCase.
    pub name: String,

    /// The fields that compose the schema.
    pub fields: Vec<Field>,
}
