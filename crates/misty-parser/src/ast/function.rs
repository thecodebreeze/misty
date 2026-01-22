use crate::ParserError;
use crate::ast::parse_data_type;
use crate::pest_parser::Rule;
use misty_ast::{DataType, Function};
use pest::iterators::Pair;

/// Parses a function declaration.
#[tracing::instrument(skip(pair))]
pub fn parse_function(pair: Pair<Rule>) -> Result<Function, ParserError> {
    let mut function_inner = pair.into_inner();

    // Extract the name of the function.
    let name = function_inner
        .next()
        .ok_or(ParserError::FunctionName)
        .inspect_err(|error| tracing::debug!(?error, "Failed to parse the function name"))?
        .as_str()
        .to_string();

    // Parse the input argument of the function.
    let input_argument_pair = function_inner
        .next()
        .ok_or(ParserError::FunctionArgument)
        .inspect_err(|error| {
            tracing::debug!(?error, "Failed to parse the function input argument")
        })?;
    let input_argument = parse_function_argument(input_argument_pair)?;

    // Parse the output argument of the function.
    let output_argument = function_inner
        .next()
        .map(|pair| parse_function_argument(pair))
        .transpose()?;

    Ok(Function {
        name,
        input: input_argument,
        output: output_argument,
    })
}

/// Parses a function argument, either input or output.
///
/// It returns a boolean flag besides the [DataType]. This flag indicates if the argument is a
/// stream or not.
fn parse_function_argument(pair: Pair<Rule>) -> Result<(bool, DataType), ParserError> {
    let mut argument_inner = pair.into_inner();
    let mut argument_pair = argument_inner
        .next()
        .ok_or(ParserError::FunctionArgument)
        .inspect_err(|error| {
            tracing::debug!(?error, "Failed to parse the function argument inner token")
        })?;

    // Extract the stream flag.
    let is_stream = if argument_pair.as_rule() == Rule::STREAM_KW {
        // In case it is, we advance to the next token, that is the ACTUAL DataType.
        argument_pair = argument_inner
            .next()
            .ok_or(ParserError::FunctionArgument)
            .inspect_err(|error| {
                tracing::debug!(?error, "Failed to parse the DataType after the STREAM_KW")
            })?;
        true
    } else {
        false
    };

    // Parse the DataType.
    let data_type_pair = argument_pair
        .into_inner()
        .next()
        .ok_or(ParserError::FunctionArgumentDataType)
        .inspect_err(|error| {
            tracing::debug!(?error, "Failed to parse the function argument DataType")
        })?;
    let data_type = parse_data_type(data_type_pair)?;

    Ok((is_stream, data_type))
}
