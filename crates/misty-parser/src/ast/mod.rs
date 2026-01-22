//! Main parser code including AST nodes parsing.
//!
//! The main entrypoint of this module is the [parse] function, but other functions are available
//! to parse individual AST nodes.
mod data_type;
mod function;
mod import;
mod interface;
mod misty_enum;
mod schema;

pub use data_type::parse_data_type;
pub use function::parse_function;
pub use import::parse_import;
pub use interface::parse_interface;
pub use misty_enum::parse_enum;
pub use schema::parse_schema;

use crate::ParserError;
use crate::pest_parser::{MistyPestParser, Rule};
use misty_ast::{Definition, File};
use pest::Parser;

#[tracing::instrument(skip(source))]
pub fn parse(source: &str) -> Result<File, ParserError> {
    // Try to parse the root rule which is our `file` grammar node.
    let mut pairs = MistyPestParser::parse(Rule::file, source)
        .inspect_err(|error| tracing::debug!(?error, "Failed to parse the root rule"))?;
    let root = pairs
        .next()
        .ok_or(ParserError::RootNode)
        .inspect_err(|error| tracing::debug!(?error, "No root node was found in the source"))?;

    // Iterate over every remaining Pairs inside the root node and extract the imports list and
    // definitions list.
    let mut imports = Vec::new();
    let mut definitions = Vec::new();
    for pair in root.into_inner() {
        match pair.as_rule() {
            Rule::import_stmt => imports.push(parse_import(pair)),
            Rule::interface_def => definitions.push(Definition::Interface(parse_interface(pair)?)),
            Rule::schema_def => definitions.push(Definition::Schema(parse_schema(pair)?)),
            Rule::enum_def => definitions.push(Definition::Enum(parse_enum(pair)?)),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    // Return the built File node.
    Ok(File {
        imports,
        definitions,
    })
}
