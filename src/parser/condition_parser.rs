#[cfg(test)]
mod tests;

use super::parse_expression;
use crate::{
    ast::{Condition, ConditionType, JoinType},
    CaosError, Rule,
};
use pest::iterators::{Pair, Pairs};

pub fn parse_condition(pairs: &mut Pairs<Rule>) -> Result<Condition, CaosError> {
    let mut c = parse_condition_single(pairs)?;
    let mut v = Vec::<(Condition, JoinType)>::new();
    while let Some((j, new_c)) = try_parse_condition_join_part(pairs)? {
        v.push((c, j));
        c = new_c;
    }
    Ok(v.into_iter().rev().fold(c, |c_rhs, (c_lhs, join_type)| {
        Condition::Combination { c_lhs: Box::new(c_lhs), c_rhs: Box::new(c_rhs), join_type }
    }))
}

fn try_parse_condition_join_part(
    pairs: &mut Pairs<Rule>,
) -> Result<Option<(JoinType, Condition)>, CaosError> {
    if let Some(p) = pairs.peek() {
        if let Ok(join_type) =  parse_condition_join(p) {
            _ = pairs.next();
            let c = parse_condition_single(pairs)?;
            return Ok(Some((join_type, c)));
        }
    }
    Ok(None)
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

fn parse_condition_single(pairs: &mut Pairs<Rule>) -> Result<Condition, CaosError> {
    let lhs = parse_expression(pairs)?;
    let cond_type = pairs
        .next()
        .ok_or_else(|| CaosError::new_end_of_stream())
        .and_then(parse_condition_operator)?;
    let rhs = parse_expression(pairs)?;
    Ok(Condition::Simple { cond_type, lhs, rhs })
}
