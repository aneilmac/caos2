#[cfg(test)]
mod tests;

use super::{parse_agent, parse_bytestring_literal, parse_decimal, parse_string, parse_variable};
use crate::{
    ast::{Anything, Condition, ConditionType, JoinType},
    CaosError, Rule,
};
use pest::iterators::Pair;

pub fn parse_anything(pair: Pair<Rule>) -> Result<Anything, CaosError> {
    match pair.as_rule() {
        Rule::variable => parse_variable(pair).map(Anything::Variable),
        Rule::string => parse_string(pair).map(Anything::String),
        Rule::literal_byte_string => parse_bytestring_literal(pair).map(Anything::ByteString),
        Rule::agent => parse_agent(pair).map(Anything::Agent),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_condition(pair: Pair<Rule>) -> Result<Condition, CaosError> {
    todo!()
    // if pair.as_rule() != Rule::condition {
    //     return Err(CaosError::new_parse_error(pair));
    // }

    // let mut it = pair.clone().into_inner();

    // let mut current: Condition = it
    //     .next()
    //     .ok_or(CaosError::new_parse_error(pair.clone()))
    //     .and_then(parse_simple_condition)?;

    // for p in it {
    //     let (j, c) = parse_condition_join_part(p)?;
    //     current = current.join(c, j);
    // }

    // Ok(current)
}

fn parse_simple_condition(pair: Pair<Rule>) -> Result<Condition, CaosError> {
    todo!()
    // if pair.as_rule() != Rule::simple_condition {
    //     return Err(CaosError::new_parse_error(pair));
    // }
    // let mut it = pair.clone().into_inner();
    // let lhs = it
    //     .next()
    //     .ok_or(CaosError::new_parse_error(pair.clone()))
    //     .and_then(parse_anything)?;
    // let cond_type = it
    //     .next()
    //     .ok_or(CaosError::new_parse_error(pair.clone()))
    //     .and_then(parse_condition_operator)?;
    // let rhs = it
    //     .next()
    //     .ok_or(CaosError::new_parse_error(pair.clone()))
    //     .and_then(parse_anything)?;
    // Ok(Condition::Simple {
    //     cond_type,
    //     lhs,
    //     rhs,
    // })
}

fn parse_condition_join_part(pair: Pair<Rule>) -> Result<(JoinType, Condition), CaosError> {
    todo!()
    // if pair.as_rule() != Rule::condition_join_part {
    //     return Err(CaosError::new_parse_error(pair));
    // }
    // let mut it = pair.clone().into_inner();
    // let join_type = it
    //     .next()
    //     .ok_or(CaosError::new_parse_error(pair.clone()))
    //     .and_then(parse_condition_join)?;
    // let condition = it
    //     .next()
    //     .ok_or(CaosError::new_parse_error(pair.clone()))
    //     .and_then(parse_simple_condition)?;
    // Ok((join_type, condition))
}

fn parse_condition_operator(pair: Pair<Rule>) -> Result<ConditionType, CaosError> {
    match pair.as_rule() {
        Rule::condition_eq => Ok(ConditionType::Eq),
        Rule::condition_ge => Ok(ConditionType::Ge),
        Rule::condition_gt => Ok(ConditionType::Gt),
        Rule::condition_le => Ok(ConditionType::Le),
        Rule::condition_ne => Ok(ConditionType::Ne),
        Rule::condition_lt => Ok(ConditionType::Lt),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

fn parse_condition_join(pair: Pair<Rule>) -> Result<JoinType, CaosError> {
    match pair.as_rule() {
        Rule::condition_and => Ok(JoinType::And),
        Rule::condition_or => Ok(JoinType::Or),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}
