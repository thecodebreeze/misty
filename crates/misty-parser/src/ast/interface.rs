use crate::ParserError;
use crate::ast::parse_function;
use crate::pest_parser::Rule;
use misty_ast::Interface;
use pest::iterators::Pair;

/// Parses an Interface definition.
#[tracing::instrument(skip(pair))]
pub fn parse_interface(pair: Pair<Rule>) -> Result<Interface, ParserError> {
    let mut inner = pair.into_inner();

    // Extract the name of the interface.
    let name = inner
        .next()
        .ok_or(ParserError::InterfaceName)
        .inspect_err(|error| tracing::debug!(?error, "Failed to parse the interface name"))?
        .as_str()
        .to_string();

    // Parse each function in the interface.
    let mut functions = Vec::new();
    for function_pair in inner {
        let function = parse_function(function_pair)?;
        functions.push(function);
    }

    Ok(Interface { name, functions })
}
