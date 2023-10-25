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
use std::vec::Vec;

pub(crate) use expression_parser_trait::ExpressionParser;
pub(crate) use expression_thunk::{ExpressionThunk, Partial};

pub fn parse_expressions<'i>(mut pairs: Pairs<'i, Rule>) -> Result<Vec<Anything>, CaosError> {
    let mut parts = Vec::<Anything>::new();
    let mut stack: ExpressionStack<'i> = ExpressionStack::new();

    while let Some(pair) = pairs.next() {
        let rule = pair.as_rule();
        let thunk: ExpressionThunk = match rule {
            Rule::literal_string => Some(
                parse_string_literal(pair.clone())
                    .map(Anything::from)?
                    .into(),
            ),
            Rule::literal_byte_string => Some(
                parse_bytestring_literal(pair.clone())
                    .map(Anything::from)?
                    .into(),
            ),
            Rule::literal_int => Some(parse_int_literal(pair.clone()).map(Anything::from)?.into()),
            Rule::literal_float => Some(
                parse_float_literal(pair.clone())
                    .map(Anything::from)?
                    .into(),
            ),
            Rule::variable_mvxx => Some(
                pair.clone()
                    .into_inner()
                    .next()
                    .ok_or(CaosError::new_parse_error(pair.clone()))
                    .and_then(parse_variable_digit)
                    .map(Variable::Mvxx)
                    .map(Anything::from)?
                    .into(),
            ),
            Rule::variable_ovxx => Some(
                pair.clone()
                    .into_inner()
                    .next()
                    .ok_or(CaosError::new_parse_error(pair.clone()))
                    .and_then(parse_variable_digit)
                    .map(Variable::Ovxx)
                    .map(Anything::from)?
                    .into(),
            ),
            Rule::variable_vaxx => Some(
                pair.clone()
                    .into_inner()
                    .next()
                    .ok_or(CaosError::new_parse_error(pair.clone()))
                    .and_then(parse_variable_digit)
                    .map(Variable::Vaxx)
                    .map(Anything::from)?
                    .into(),
            ),
            _ => None,
        }
        .or_else(|| Agent::parse_thunk(pair.clone()))
        .or_else(|| Float::parse_thunk(pair.clone()))
        .or_else(|| Integer::parse_thunk(pair.clone()))
        .or_else(|| SString::parse_thunk(pair.clone()))
        .or_else(|| Variable::parse_thunk(pair.clone()))
        .ok_or_else(|| CaosError::new_parse_error(pair))?;

        let res = stack.push(thunk)?;
        if let Some(a) = res {
            parts.push(a);
        }
    }

    match stack.root.last() {
        None => Ok(parts),
        Some(p) => Err(CaosError::new_arg_count_error(
            p.target_args,
            p.arg_parts.len(),
            p.origin.line_col(),
        )),
    }
}

fn parse_variable_digit(pair: Pair<Rule>) -> Result<u8, CaosError> {
    if pair.as_rule() != Rule::variable_digit {
        return Err(CaosError::new_parse_error(pair));
    }
    pair.as_str()
        .parse::<u8>()
        .map_err(|_| CaosError::new_parse_error(pair))
}
