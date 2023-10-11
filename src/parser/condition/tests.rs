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
    for p in CaosParser::parse(Rule::condition, "MV32 EQ MV23 OR 1 GT 2 AND HAND <> EMID")
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
                        rhs: Anything::String(SString::Emid)
                    }),
                    join_type: JoinType::And
                }),
                join_type: JoinType::Or
            }
        );
    }
}
