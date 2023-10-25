#[cfg(test)]
mod tests;

use super::parse_expressions;
use crate::{
    ast::{Condition, ConditionType, JoinType},
    CaosError, Rule,
};
use pest::iterators::{Pair, Pairs};

pub fn parse_condition(mut it: Pairs<Rule>) -> Result<Condition, CaosError> {
    let (mut j, mut c) = parse_condition_join_part(&mut it)?;
    loop {
        match j {
            Some(ji) => {
                let (j_next, c_next) = parse_condition_join_part(&mut it)?;
                c = Condition::Combination {
                    c_lhs: Box::new(c_next),
                    c_rhs: Box::new(c),
                    join_type: ji,
                };
                j = j_next;
            }
            None => break,
        }
    }

    Ok(c)
}

fn parse_condition_join_part(
    pairs: &mut Pairs<Rule>,
) -> Result<(Option<JoinType>, Condition), CaosError> {
    let condition = pairs.next_back().map(parse_condition_single).unwrap()?;

    let join_type = pairs.next_back().map(parse_condition_join).transpose()?;

    Ok((join_type, condition))
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

fn parse_condition_single(pair: Pair<Rule>) -> Result<Condition, CaosError> {
    if pair.as_rule() != Rule::condition_single {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let lhs = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(|it| parse_expressions(it.into_inner()))?;

    if lhs.len() != 1 {
        return Err(CaosError::new_arg_count_error(
            1,
            lhs.len(),
            pair.line_col(),
        ));
    }

    let cond_type = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_condition_operator)?;

    let rhs = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(|it| parse_expressions(it.into_inner()))?;

    if rhs.len() != 1 {
        return Err(CaosError::new_arg_count_error(
            1,
            rhs.len(),
            pair.line_col(),
        ));
    }

    Ok(Condition::Simple {
        cond_type,
        lhs: lhs.into_iter().next().unwrap(),
        rhs: rhs.into_iter().next().unwrap(),
    })
}
