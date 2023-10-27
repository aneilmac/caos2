#[cfg(test)]
mod tests;

mod command_parser_trait;
mod command_thunk;
mod expression_parser_trait;
mod expression_stack;
mod expression_thunk;
mod partial;

use crate::{
    ast::{Agent, Anything, Command, Float, Integer, SString, Variable},
    parser::base::{
        parse_bytestring_literal, parse_float_literal, parse_int_literal, parse_string_literal,
    },
    CaosError, Rule,
};
use expression_stack::ExpressionStack;
use pest::iterators::{Pair, Pairs};
use std::vec::Vec;

pub(crate) use command_parser_trait::CommandParser;
pub(crate) use command_thunk::CommandThunk;
pub(crate) use expression_parser_trait::ExpressionParser;
pub(crate) use expression_thunk::ExpressionThunk;
pub(crate) use partial::Partial;

pub fn parse_commands<'i>(mut pairs: Pairs<'i, Rule>) -> Result<Vec<Command>, CaosError> {
    let mut commands = Vec::<Command>::new();

    while let Some(pair) = pairs.next() {
        let mut thunk: CommandThunk = 
            find_command_match(pair)?;

        loop {
            if thunk.is_ready() {
                let c = thunk.complete()?;
                commands.push(c);
                break;
            } else {
                match thunk {
                    CommandThunk::CompletedCommand(..) => unreachable!(),
                    CommandThunk::PartialCommand(ref mut p) => {
                        let arg = parse_expression(&mut pairs)?;
                        p.arg_parts.push(arg);
                    }
                }
            }
        }
    }
    Ok(commands)
}

fn find_command_match(pair: Pair<Rule>) -> Result<CommandThunk, CaosError> {
    todo!()
}

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
        None =>  Err(CaosError::new_end_of_stream())
    }
}

fn find_expression_match(pair: Pair<Rule>) -> Result<ExpressionThunk, CaosError> {
    let rule = pair.as_rule();
    match rule {
        Rule::literal_string => Some(
            parse_string_literal(pair.clone())
                .map(Anything::from)
                .map(|v| ExpressionThunk::CompletedExpression(pair.clone(), v))?,
        ),
        Rule::literal_byte_string => Some(
            parse_bytestring_literal(pair.clone())
                .map(Anything::from)
                .map(|v| ExpressionThunk::CompletedExpression(pair.clone(), v))?,
        ),
        Rule::literal_int => Some(
            parse_int_literal(pair.clone())
                .map(Anything::from)
                .map(|v| ExpressionThunk::CompletedExpression(pair.clone(), v))?,
        ),
        Rule::literal_float => Some(
            parse_float_literal(pair.clone())
                .map(Anything::from)
                .map(|v| ExpressionThunk::CompletedExpression(pair.clone(), v))?,
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
    Ok(ExpressionThunk::CompletedExpression(pair, v))
}

fn parse_variable_digit(pair: Pair<Rule>) -> Result<u8, CaosError> {
    if pair.as_rule() != Rule::variable_digit {
        return Err(CaosError::new_parse_error(pair));
    }
    pair.as_str()
        .parse::<u8>()
        .map_err(|_| CaosError::new_parse_error(pair))
}
