use crate::{
    ast::Float, ast::FloatArg, parse_float_literal, parse_int, parse_variable, CaosError, Rule,
};
use pest::iterators::Pair;

pub fn parse_float(pair: Pair<Rule>) -> Result<Float, CaosError> {
    if pair.as_rule() != Rule::float {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::literal_float => parse_float_literal(pair).map(|f| Float::Literal(f.into())),
        _ => todo!("Implement floats"),
    }
}

pub fn parse_float_arg(pair: Pair<Rule>) -> Result<FloatArg, CaosError> {
    match pair.as_rule() {
        Rule::float => parse_float(pair).map(FloatArg::Primary),
        Rule::int => parse_int(pair).map(FloatArg::Castable),
        Rule::variable => parse_variable(pair).map(FloatArg::Variable),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}
