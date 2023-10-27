#[cfg(test)]
mod tests;

mod expression_parser_trait;
mod expression_stack;
mod expression_thunk;

use crate::{
    ast::{Agent, Anything, Float, Integer, SString, Variable},
    parser::base::{
        parse_bytestring_literal, parse_float_literal, parse_int_literal, parse_string_literal,
    },
    CaosError, Rule,
};
use expression_stack::ExpressionStack;
use pest::iterators::{Pair, Pairs};

use super::Partial;
pub(crate) use expression_parser_trait::ExpressionParser;
pub(crate) use expression_thunk::ExpressionThunk;

pub fn parse_expression<'i>(pairs: &'_ mut Pairs<'i, Rule>) -> Result<Anything, CaosError> {
    let mut expression_stack = ExpressionStack::new();
    while let Some(pair) = pairs.next() {
        let thunk: ExpressionThunk = find_expression_match(pair)?;
        let res = expression_stack.push(thunk)?;
        if let Some(res) = res {
            return Ok(res);
        }
    }

    match expression_stack.root.into_iter().last() {
        Some(e) => Err(CaosError::new_parse_error(e.origin)),
        None => Err(CaosError::new_end_of_stream()),
    }
}

fn find_expression_match(pair: Pair<Rule>) -> Result<ExpressionThunk, CaosError> {
    let rule = pair.as_rule();
    match rule {
        Rule::literal_string => Some(
            parse_string_literal(pair.clone())
                .map(Anything::from)
                .map(|v| ExpressionThunk::Completed(pair.clone(), v))?,
        ),
        Rule::literal_byte_string => Some(
            parse_bytestring_literal(pair.clone())
                .map(Anything::from)
                .map(|v| ExpressionThunk::Completed(pair.clone(), v))?,
        ),
        Rule::literal_int => Some(
            parse_int_literal(pair.clone())
                .map(Anything::from)
                .map(|v| ExpressionThunk::Completed(pair.clone(), v))?,
        ),
        Rule::literal_float => Some(
            parse_float_literal(pair.clone())
                .map(Anything::from)
                .map(|v| ExpressionThunk::Completed(pair.clone(), v))?,
        ),
        Rule::variable_mvxx => Some(parse_variable(pair.clone(), Variable::Mvxx)?),
        Rule::variable_ovxx => Some(parse_variable(pair.clone(), Variable::Ovxx)?),
        Rule::variable_vaxx => Some(parse_variable(pair.clone(), Variable::Vaxx)?),
        _ => None,
    }
    .or_else(|| Variable::parse_thunk(pair.clone()))
    .or_else(|| Agent::parse_thunk(pair.clone()))
    .or_else(|| Float::parse_thunk(pair.clone()))
    .or_else(|| Integer::parse_thunk(pair.clone()))
    .or_else(|| SString::parse_thunk(pair.clone()))
    .ok_or_else(|| CaosError::new_parse_error(pair))
}

fn parse_variable<F>(pair: Pair<Rule>, f: F) -> Result<ExpressionThunk, CaosError>
where
    F: FnOnce(u8) -> Variable,
{
    let v = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_variable_digit)
        .map(f)
        .map(Anything::from)?;
    Ok(ExpressionThunk::Completed(pair, v))
}

fn parse_variable_digit(pair: Pair<Rule>) -> Result<u8, CaosError> {
    if pair.as_rule() != Rule::variable_digit {
        return Err(CaosError::new_parse_error(pair));
    }
    pair.as_str()
        .parse::<u8>()
        .map_err(|_| CaosError::new_parse_error(pair))
}
