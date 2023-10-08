use crate::{ast::SString, ast::SStringArg, parse_string_literal, parse_variable, CaosError, Rule};
use pest::iterators::Pair;

pub fn parse_string(pair: Pair<Rule>) -> Result<SString, CaosError> {
    if (pair.as_rule() != Rule::string) {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::literal_string => parse_string_literal(pair).map(SString::Literal),
        _ => todo!("Implement strings"),
    }
}

pub fn parse_string_arg(pair: Pair<Rule>) -> Result<SStringArg, CaosError> {
    match pair.as_rule() {
        Rule::string => parse_string(pair).map(SStringArg::String),
        Rule::variable => parse_variable(pair).map(SStringArg::Variable),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}
