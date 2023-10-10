use crate::{
    ast::IntArg, ast::Integer, parse_float, parse_int_literal, parse_variable, CaosError, Rule,
};
use pest::iterators::Pair;

pub fn parse_int(pair: Pair<Rule>) -> Result<Integer, CaosError> {
    if pair.as_rule() != Rule::int {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::literal_int => parse_int_literal(pair).map(Integer::Literal),
        Rule::int_attr => Ok(Integer::Attr),
        _ => todo!("Implement ints"),
    }
}

pub fn parse_int_arg(pair: Pair<Rule>) -> Result<IntArg, CaosError> {
    match pair.as_rule() {
        Rule::int => parse_int(pair).map(IntArg::Primary),
        Rule::float => parse_float(pair).map(IntArg::Castable),
        Rule::variable => parse_variable(pair).map(IntArg::Variable),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}
