use crate::pest_parser::Rule;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Pest Parser Error: {0}")]
    Pest(#[from] pest::error::Error<Rule>),

    #[error("No root AST node was found in the source")]
    RootNode,

    #[error("The Enum name is invalid or is missing")]
    EnumName,

    #[error("The Schema name is invalid or is missing")]
    SchemaName,

    #[error("The Schema Field name is invalid or is missing")]
    FieldName,

    #[error("The Interface name is invalid or is missing")]
    InterfaceName,

    #[error("The Function name is invalid or is missing")]
    FunctionName,

    #[error("The Schema Field type is invalid or is missing")]
    FieldDataType,

    #[error("The DataType declared is invalid")]
    DataType,

    #[error("The DataType container name is invalid")]
    DataTypeContainerName,

    #[error("The DataType container inner type is invalid")]
    DataTypeContainerInnerType,

    #[error("The Function argument is invalid or is missing")]
    FunctionArgument,

    #[error("The Function Argument type is invalid or is missing")]
    FunctionArgumentDataType,
}
