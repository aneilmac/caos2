use super::{parse_float, parse_int, parse_variable};
use crate::{ast::Decimal, ast::DecimalArg, CaosError, ErrorType, Rule};
use pest::iterators::Pair;

pub fn parse_decimal(pair: Pair<Rule>) -> Result<Decimal, CaosError> {
    if pair.as_rule() != Rule::decimal {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::float => parse_float(pair).map(Decimal::Float),
        Rule::int => parse_int(pair).map(Decimal::Integer),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_decimal_arg(pair: Pair<Rule>) -> Result<DecimalArg, CaosError> {
    match pair.as_rule() {
        Rule::decimal => parse_decimal(pair).map(DecimalArg::Decimal),
        Rule::variable => parse_variable(pair).map(DecimalArg::Variable),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Float, Integer, Variable},
        parser::CaosParser,
    };
    use pest::Parser;

    use super::*;

    #[test]
    fn test_decimal_arg() {
        for p in CaosParser::parse(Rule::decimal_arg, "MV00").expect("Parsed") {
            assert_eq!(
                parse_decimal_arg(p).expect("Parsed variable"),
                DecimalArg::Variable(Variable::Mvxx(0)),
            );
        }
        for p in CaosParser::parse(Rule::decimal_arg, "3").expect("Parsed") {
            assert_eq!(
                parse_decimal_arg(p).expect("Parsed variable"),
                DecimalArg::Decimal(Decimal::Integer(Integer::Literal(3))),
            );
        }
        for p in CaosParser::parse(Rule::decimal_arg, "3.2").expect("Parsed") {
            assert_eq!(
                parse_decimal_arg(p).expect("Parsed variable"),
                DecimalArg::Decimal(Decimal::Float(Float::Literal(3.2f32.into()))),
            );
        }
    }
}
