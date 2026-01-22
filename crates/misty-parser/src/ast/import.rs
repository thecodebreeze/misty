use crate::pest_parser::Rule;
use pest::iterators::Pair;

/// The import statement is very simple to parse; we get each token in it and join every
/// part with a dot.
///
/// This creates an import string equal to how it's defined: `my_module.a.b.c`.
pub fn parse_import(pair: Pair<Rule>) -> String {
    pair.into_inner()
        .map(|pair| pair.as_str())
        .collect::<Vec<_>>()
        .join(".")
}
