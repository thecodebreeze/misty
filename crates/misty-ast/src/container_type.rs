/// Available types of containers.
#[derive(Clone)]
pub enum ContainerType {
    /// Vectors are Rust's [Vec],
    ///
    /// They are represented as an `Array` in JavaScript.
    Vec,

    /// Options are Rust's [Option].
    ///
    /// They are represented as nullable in JavaScript.
    Option,
}
