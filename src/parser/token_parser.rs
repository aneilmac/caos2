use crate::{
    ast::{
        Agent, Anything, Command, Condition, Float, Integer, JoinType, Label, SString,
        ScriptDefinition, Variable,
    },
    parser::base::{
        parse_bytestring_literal, parse_float_literal, parse_int_literal, parse_string_literal,
    },
    CaosError, Rule,
};
use pest::iterators::{Pair, Pairs};
use std::vec::Vec;

pub(crate) trait ExpressionParser {
    fn parse_thunk<'i>(pair: Pair<'i, Rule>) -> Option<ExpressionThunk<'i>>;
}

pub(crate) enum ExpressionThunk<'i> {
    Completed(Anything),
    Partial {
        origin: Pair<'i, Rule>,
        arg_parts: Vec<Anything>,
        target_args: usize,
        complete_method: Box<dyn Fn(Pair<'i, Rule>, Vec<Anything>) -> Result<Anything, CaosError>>,
    },
}

impl From<Anything> for ExpressionThunk<'_> {
    fn from(value: Anything) -> Self {
        ExpressionThunk::Completed(value)
    }
}

pub fn parse_expressions(mut pairs: Pairs<Rule>) -> Result<Vec<Anything>, CaosError> {
    let mut parts = Vec::<Anything>::new();
    while let Some(pair) = pairs.next() {
        let thunk: Option<ExpressionThunk> = match pair.clone().as_rule() {
            Rule::literal_string => Some(parse_string_literal(pair.clone()).map(Anything::from)?.into()),
            Rule::literal_byte_string => {
                Some(parse_bytestring_literal(pair.clone()).map(Anything::from)?.into())
            }
            Rule::literal_int => Some(parse_int_literal(pair.clone()).map(Anything::from)?.into()),
            Rule::literal_float => Some(parse_float_literal(pair.clone()).map(Anything::from)?.into()),
            _ => None,
        }
        .or_else(|| Agent::parse_thunk(pair.clone()))
        .or_else(|| Float::parse_thunk(pair.clone()))
        .or_else(|| Integer::parse_thunk(pair.clone()))
        .or_else(|| SString::parse_thunk(pair.clone()))
        .or_else(|| Variable::parse_thunk(pair));
    }
    todo!()
}
