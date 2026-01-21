use crate::DataType;

/// Functions are one of the core building blocks of Misty.
///
/// They are composed of their names, input and output arguments; both of which are singular.
///
/// Input and Output arguments can be streamable, with the Output argument specifically being a
/// `None` for fire-and-forget kinda functions.
///
/// Behind the scenes, it does not matter because all inputs and outputs are handled as a
/// Stream! But the flag helps optimize and shifts the code generation a bit.
#[derive(Clone)]
pub struct Function {
    /// Name of the function.
    ///
    /// Must be in snake_case.
    pub name: String,

    /// Input argument of the function.
    ///
    /// The boolean indicates if this is a stream or not.
    pub input: (bool, DataType),

    /// Output argument of the function.
    ///
    /// It's optional because this allows us to implement fire-and-forget.
    ///
    /// The boolean indicates if this is a stream or not.
    pub output: Option<(bool, DataType)>,
}
