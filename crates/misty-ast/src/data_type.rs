use crate::ContainerType;

/// Available types of data fields and arguments can have.
#[derive(Clone)]
pub enum DataType {
    /// Basic data types that every language has, or can have with some adaptations.
    ///
    /// We support the most important Rust primitives out-of-the-box.
    Primitive(String),

    /// User-defined types.
    ///
    /// They can be either Local or External. By specification, user-defined types that are external
    /// must reference the entire module path for resolution. Although verbose, this is to ensure no
    /// name clashes and to make intent clear.
    ///
    /// This also permits that
    UserType(String),

    /// Containers are types that can hold other types.
    Container(ContainerType, Box<DataType>),
}
