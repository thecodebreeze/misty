use crate::ParserError;
use crate::pest_parser::Rule;
use misty_ast::DataType;
use pest::iterators::Pair;

/// Parses a DataType component.
#[tracing::instrument(skip(pair))]
pub fn parse_data_type(pair: Pair<Rule>) -> Result<DataType, ParserError> {
    // Extract the inner token for the DataType definition.
    let data_type_pair = pair
        .into_inner()
        .next()
        .ok_or(ParserError::DataType)
        .inspect_err(|error| tracing::debug!(?error, "Failed to parse the DataType"))?;

    // Parse the token as a Rule to match the proper type.
    match data_type_pair.as_rule() {
        // A primitive DataType is just a string.
        Rule::primitive => Ok(DataType::Primitive(data_type_pair.as_str().to_string())),

        // A user-defined DataType is just a string. We do not need to resolve the type in this
        // step. This will be done later when we validate the File against the Workspace.
        Rule::user_type => Ok(DataType::UserType(data_type_pair.as_str().to_string())),

        // Parsing a container data type requires recursing to resolve all the type sequence.
        Rule::container => {
            let mut container_inner = data_type_pair.into_inner();

            // Extract the name of the container type (vec/option).
            let container_type_name = container_inner
                .next()
                .ok_or(ParserError::DataTypeContainerName)
                .inspect_err(|error| {
                    tracing::debug!(?error, "Failed to parse the container type name")
                })?;

            // Recursively parse the DataType inside the container.
            let container_inner_data_type_pair = container_inner
                .next()
                .ok_or(ParserError::DataTypeContainerInnerType)
                .inspect_err(|error| {
                    tracing::debug!(?error, "Failed to parse the container inner type")
                })?;
            let inner_data_type = parse_data_type(container_inner_data_type_pair)?;

            // Match the container type name to build the DataType correctly.
            let container_type = match container_type_name.as_str() {
                "vec" => {
                    DataType::Container(misty_ast::ContainerType::Vec, Box::new(inner_data_type))
                }
                "option" => {
                    DataType::Container(misty_ast::ContainerType::Option, Box::new(inner_data_type))
                }
                _ => unreachable!(),
            };

            Ok(container_type)
        }
        _ => unreachable!(),
    }
}
