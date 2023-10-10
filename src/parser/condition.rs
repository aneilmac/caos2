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
        Rule::decimal => parse_decimal(pair).map(Anything::Decimal),
        Rule::literal_byte_string => parse_bytestring_literal(pair).map(Anything::ByteString),
        Rule::agent => parse_agent(pair).map(Anything::Agent),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_condition(pair: Pair<Rule>) -> Result<Condition, CaosError> {
    if pair.as_rule() != Rule::condition {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let mut current: Condition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_simple_condition)?;

    for p in it {
        let (j, c) = parse_condition_join_part(p)?;
        current = current.join(c, j);
    }

    Ok(current)
}

fn parse_simple_condition(pair: Pair<Rule>) -> Result<Condition, CaosError> {
    if pair.as_rule() != Rule::simple_condition {
        return Err(CaosError::new_parse_error(pair));
    }
    let mut it = pair.clone().into_inner();
    let lhs = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_anything)?;
    let cond_type = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_condition_operator)?;
    let rhs = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_anything)?;
    Ok(Condition::Simple {
        cond_type,
        lhs,
        rhs,
    })
}

fn parse_condition_join_part(pair: Pair<Rule>) -> Result<(JoinType, Condition), CaosError> {
    if pair.as_rule() != Rule::condition_join_part {
        return Err(CaosError::new_parse_error(pair));
    }
    let mut it = pair.clone().into_inner();
    let join_type = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_condition_join)?;
    let condition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_simple_condition)?;
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

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Agent, Decimal, Float, Integer, SString, Variable},
        parser::CaosParser,
    };
    use pest::Parser;

    use super::*;

    #[test]
    fn test_anything() {
        for p in CaosParser::parse(Rule::anything, "MV32").expect("Parsed") {
            assert_eq!(
                parse_anything(p).expect("Parsed anything"),
                Anything::Variable(Variable::Mvxx(32))
            );
        }
        for p in CaosParser::parse(Rule::anything, "HAND").expect("Parsed") {
            assert_eq!(
                parse_anything(p).expect("Parsed anything"),
                Anything::String(SString::Hand)
            );
        }
        for p in CaosParser::parse(Rule::anything, "3.23").expect("Parsed") {
            assert_eq!(
                parse_anything(p).expect("Parsed anything"),
                Anything::Decimal(Decimal::Float(Float::Literal(3.23f32.into())))
            );
        }
        for p in CaosParser::parse(Rule::anything, "3").expect("Parsed") {
            assert_eq!(
                parse_anything(p).expect("Parsed anything"),
                Anything::Decimal(Decimal::Integer(Integer::Literal(3)))
            );
        }
        for p in CaosParser::parse(Rule::anything, "[3 2]").expect("Parsed") {
            assert_eq!(
                parse_anything(p).expect("Parsed anything"),
                Anything::ByteString(vec![3, 2].into())
            );
        }
        for p in CaosParser::parse(Rule::anything, "HOTS").expect("Parsed") {
            assert_eq!(
                parse_anything(p).expect("Parsed anything"),
                Anything::Agent(Agent::Hots)
            );
        }
    }

    #[test]
    fn test_condition_eq() {
        for p in CaosParser::parse(Rule::condition, "MV32 EQ MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Eq,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
        for p in CaosParser::parse(Rule::condition, "MV32 = MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Eq,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
    }

    #[test]
    fn test_condition_ge() {
        for p in CaosParser::parse(Rule::condition, "MV32 GE MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Ge,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
        for p in CaosParser::parse(Rule::condition, "MV32 >= MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Ge,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
    }

    #[test]
    fn test_condition_gt() {
        for p in CaosParser::parse(Rule::condition, "MV32 GT MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Gt,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
        for p in CaosParser::parse(Rule::condition, "MV32 > MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Gt,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
    }

    #[test]
    fn test_condition_le() {
        for p in CaosParser::parse(Rule::condition, "MV32 le MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Le,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
        for p in CaosParser::parse(Rule::condition, "MV32 <= MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Le,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
    }

    #[test]
    fn test_condition_ne() {
        for p in CaosParser::parse(Rule::condition, "MV32 ne MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Ne,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
        for p in CaosParser::parse(Rule::condition, "MV32 <> MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Ne,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
    }

    #[test]
    fn test_condition_lt() {
        for p in CaosParser::parse(Rule::condition, "MV32 LT MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Lt,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
        for p in CaosParser::parse(Rule::condition, "MV32 < MV23").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Simple {
                    cond_type: ConditionType::Lt,
                    lhs: Anything::Variable(Variable::Mvxx(32)),
                    rhs: Anything::Variable(Variable::Mvxx(23))
                }
            );
        }
    }

    #[test]
    fn test_condition_and() {
        for p in CaosParser::parse(Rule::condition, "MV32 EQ MV23 AND 1 GT 2").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Combination {
                    c_lhs: Box::new(Condition::Simple {
                        cond_type: ConditionType::Eq,
                        lhs: Anything::Variable(Variable::Mvxx(32)),
                        rhs: Anything::Variable(Variable::Mvxx(23))
                    }),
                    c_rhs: Box::new(Condition::Simple {
                        cond_type: ConditionType::Gt,
                        lhs: Anything::Decimal(Decimal::Integer(Integer::Literal(1))),
                        rhs: Anything::Decimal(Decimal::Integer(Integer::Literal(2)))
                    }),
                    join_type: JoinType::And
                }
            );
        }
    }

    #[test]
    fn test_condition_or() {
        for p in CaosParser::parse(Rule::condition, "MV32 EQ MV23 OR 1 GT 2").expect("Parsed") {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Combination {
                    c_lhs: Box::new(Condition::Simple {
                        cond_type: ConditionType::Eq,
                        lhs: Anything::Variable(Variable::Mvxx(32)),
                        rhs: Anything::Variable(Variable::Mvxx(23))
                    }),
                    c_rhs: Box::new(Condition::Simple {
                        cond_type: ConditionType::Gt,
                        lhs: Anything::Decimal(Decimal::Integer(Integer::Literal(1))),
                        rhs: Anything::Decimal(Decimal::Integer(Integer::Literal(2)))
                    }),
                    join_type: JoinType::Or
                }
            );
        }
    }

    #[test]
    fn test_condition_3way() {
        for p in CaosParser::parse(Rule::condition, "MV32 EQ MV23 OR 1 GT 2 AND HAND <> HAND")
            .expect("Parsed")
        {
            assert_eq!(
                parse_condition(p).expect("Parsed condition"),
                Condition::Combination {
                    c_lhs: Box::new(Condition::Simple {
                        cond_type: ConditionType::Eq,
                        lhs: Anything::Variable(Variable::Mvxx(32)),
                        rhs: Anything::Variable(Variable::Mvxx(23))
                    }),
                    c_rhs: Box::new(Condition::Combination {
                        c_lhs: Box::new(Condition::Simple {
                            cond_type: ConditionType::Gt,
                            lhs: Anything::Decimal(Decimal::Integer(Integer::Literal(1))),
                            rhs: Anything::Decimal(Decimal::Integer(Integer::Literal(2)))
                        }),
                        c_rhs: Box::new(Condition::Simple {
                            cond_type: ConditionType::Ne,
                            lhs: Anything::String(SString::Hand),
                            rhs: Anything::String(SString::Hand)
                        }),
                        join_type: JoinType::And
                    }),
                    join_type: JoinType::Or
                }
            );
        }
    }
}
