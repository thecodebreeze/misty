use crate::ParserError;
use crate::pest_parser::Rule;
use misty_ast::Enum;
use pest::iterators::Pair;

/// Parses an Enum definition.
///
/// Returns a [ParserError::EnumName] error if there are problems parsing the name of the
/// Enum type.
#[tracing::instrument(skip(pair))]
pub fn parse_enum(pair: Pair<Rule>) -> Result<Enum, ParserError> {
    let mut inner = pair.into_inner();

    // Extract the name of this Enum.
    let name = inner
        .next()
        .ok_or(ParserError::EnumName)
        .inspect_err(|error| tracing::debug!(?error, "Failed to parse the Enum type name"))?
        .as_str()
        .to_string();

    // Extract all the variants of this Enum.
    let variants = inner.map(|pair| pair.as_str().to_string()).collect();

    Ok(Enum { name, variants })
}
