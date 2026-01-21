use crate::DataType;

/// Fields are the building blocks of schemas.
#[derive(Clone)]
pub struct Field {
    /// The name of the field.
    ///
    /// Must be in snake_case.
    pub name: String,

    /// The type of the field.
    pub field_type: DataType,
}
