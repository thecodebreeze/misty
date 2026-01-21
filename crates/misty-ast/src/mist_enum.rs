/// Enums are similar to Protobuf's enums.
///
/// Unlike the other definitions, Misty Enum's are very similar to C and TypeScript enums in the
/// way they are laid out, they are represented as a Rust `u16`.
///
/// Also, it differs from Protobuf's enums because they don't need numbering, this is automatically
/// inferred.
#[derive(Clone)]
pub struct Enum {
    /// The name of the enum.
    ///
    /// Must be in PascalCase.
    pub name: String,

    /// The many variants that compose the enum.
    ///
    /// Every variant must be in SCREAMING_SNAKE_CASE.
    pub variants: Vec<String>,
}
