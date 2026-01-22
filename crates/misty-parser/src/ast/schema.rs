use crate::ParserError;
use crate::ast::parse_data_type;
use crate::pest_parser::Rule;
use misty_ast::{Field, Schema};
use pest::iterators::Pair;

/// Parses a Schema definition.
#[tracing::instrument(skip(pair))]
pub fn parse_schema(pair: Pair<Rule>) -> Result<Schema, ParserError> {
    let mut inner = pair.into_inner();

    // Extract the Schema type name.
    let name = inner
        .next()
        .ok_or(ParserError::SchemaName)
        .inspect_err(|error| tracing::debug!(?error, "Failed to parse the Schema type name"))?
        .as_str()
        .to_string();

    // Parse the fields declared by this Schema.
    let mut fields = Vec::new();
    for field_pair in inner {
        let mut field_inner = field_pair.into_inner();

        // Extract the Field name.
        let name = field_inner
            .next()
            .ok_or(ParserError::FieldName)
            .inspect_err(|error| tracing::debug!(?error, "Failed to parse the Field type name"))?
            .as_str()
            .to_string();

        // Parse the DataType of this Field.
        let field_type_pair = field_inner
            .next()
            .ok_or(ParserError::FieldDataType)
            .inspect_err(|error| tracing::debug!(?error, "Failed to parse the Field type"))?;
        let field_type = parse_data_type(field_type_pair)?;

        // Add this Field to the list of fields.
        fields.push(Field { name, field_type });
    }

    Ok(Schema { name, fields })
}
