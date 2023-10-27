use crate::{
    ast::{Integer, SString, Variable},
    parser::CaosParser,
};
use pest::Parser;

use super::*;

fn parse_condition_input(input: &str) -> Condition {
    let mut p = CaosParser::parse(Rule::tokens, input).expect("Parsed");
    parse_condition(&mut p).expect("Successful parse")
}

#[test]
fn test_condition_join_fail() {
    let mut p =
        CaosParser::parse(Rule::tokens, "    MV32 EQ MV23 AND").expect("Parsed successfully");
    parse_condition(&mut p).expect_err("Should fail incomplete");

    let mut p =
        CaosParser::parse(Rule::tokens, "    MV32 EQ MV23 AND MV00").expect("Parsed successfully");
    parse_condition(&mut p).expect_err("Should fail incomplete");

    let mut p = CaosParser::parse(Rule::tokens, "    MV32 EQ MV23 AND MV00 GE")
        .expect("Parsed successfully");
    parse_condition(&mut p).expect_err("Should fail incomplete");

    let mut p = CaosParser::parse(Rule::tokens, "AND MV32 EQ MV23").expect("Parsed successfully");
    parse_condition(&mut p).expect_err("Should fail incomplete");
}

#[test]
fn test_condition_compare_fail() {
    let mut p = CaosParser::parse(Rule::tokens, "EQ MV23").expect("Parsed successfully");
    parse_condition(&mut p).expect_err("Should fail incomplete");

    let mut p = CaosParser::parse(Rule::tokens, "MV32 EQ").expect("Parsed successfully");
    parse_condition(&mut p).expect_err("Should fail incomplete");
}

#[test]
fn test_condition_expression_fail() {
    let mut p = CaosParser::parse(Rule::tokens, "MV32 MV32 EQ MV23").expect("Parsed successfully");
    parse_condition(&mut p).expect_err("Should fail too many expressions LHS");
}

#[test]
fn test_condition_eq() {
    assert_eq!(
        parse_condition_input("MV32 EQ MV23"),
        Condition::Simple {
            cond_type: ConditionType::Eq,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into(),
        }
    );
    assert_eq!(
        parse_condition_input("MV32 = MV23"),
        Condition::Simple {
            cond_type: ConditionType::Eq,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into(),
        }
    );
}

#[test]
fn test_condition_ge() {
    assert_eq!(
        parse_condition_input("MV32 GE MV23"),
        Condition::Simple {
            cond_type: ConditionType::Ge,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into(),
        }
    );
    assert_eq!(
        parse_condition_input("MV32 >= MV23"),
        Condition::Simple {
            cond_type: ConditionType::Ge,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into(),
        }
    );
}

#[test]
fn test_condition_gt() {
    assert_eq!(
        parse_condition_input("MV32 GT MV23"),
        Condition::Simple {
            cond_type: ConditionType::Gt,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into()
        }
    );
    assert_eq!(
        parse_condition_input("MV32 > MV23"),
        Condition::Simple {
            cond_type: ConditionType::Gt,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into()
        }
    );
}

#[test]
fn test_condition_le() {
    assert_eq!(
        parse_condition_input("MV32 LE MV23"),
        Condition::Simple {
            cond_type: ConditionType::Le,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into()
        }
    );
    assert_eq!(
        parse_condition_input("MV32 <= MV23"),
        Condition::Simple {
            cond_type: ConditionType::Le,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into()
        }
    );
}

#[test]
fn test_condition_ne() {
    assert_eq!(
        parse_condition_input("MV32 NE MV23"),
        Condition::Simple {
            cond_type: ConditionType::Ne,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into()
        }
    );
    assert_eq!(
        parse_condition_input("MV32 <> MV23"),
        Condition::Simple {
            cond_type: ConditionType::Ne,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into()
        }
    );
}

#[test]
fn test_condition_lt() {
    assert_eq!(
        parse_condition_input("MV32 LT MV23"),
        Condition::Simple {
            cond_type: ConditionType::Lt,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into()
        }
    );
    assert_eq!(
        parse_condition_input("MV32 < MV23"),
        Condition::Simple {
            cond_type: ConditionType::Lt,
            lhs: Variable::Mvxx(32).into(),
            rhs: Variable::Mvxx(23).into()
        }
    );
}

#[test]
fn test_condition_and() {
    assert_eq!(
        parse_condition_input("MV32 EQ MV23 AND 1 GT 2"),
        Condition::Combination {
            c_lhs: Box::new(Condition::Simple {
                cond_type: ConditionType::Eq,
                lhs: Variable::Mvxx(32).into(),
                rhs: Variable::Mvxx(23).into(),
            }),
            c_rhs: Box::new(Condition::Simple {
                cond_type: ConditionType::Gt,
                lhs: 1.into(),
                rhs: 2.into()
            }),
            join_type: JoinType::And
        }
    );
}

#[test]
fn test_condition_or() {
    assert_eq!(
        parse_condition_input("MV32 EQ MV23 OR 1 GT 2"),
        Condition::Combination {
            c_lhs: Box::new(Condition::Simple {
                cond_type: ConditionType::Eq,
                lhs: Variable::Mvxx(32).into(),
                rhs: Variable::Mvxx(23).into(),
            }),
            c_rhs: Box::new(Condition::Simple {
                cond_type: ConditionType::Gt,
                lhs: 1.into(),
                rhs: 2.into()
            }),
            join_type: JoinType::Or
        }
    );
}

#[test]
fn test_condition_3way() {
    assert_eq!(
        parse_condition_input("MUTE 1 2 EQ MV23 OR 1 GT 2 AND HAND <> EMID"),
        Condition::Combination {
            c_lhs: Box::new(Condition::Simple {
                cond_type: ConditionType::Eq,
                lhs: Integer::Mute {
                    and_mask: Box::new(1.into()),
                    eor_mask: (Box::new(2.into()))
                }
                .into(),
                rhs: Variable::Mvxx(23).into(),
            }),
            c_rhs: Box::new(Condition::Combination {
                c_lhs: Box::new(Condition::Simple {
                    cond_type: ConditionType::Gt,
                    lhs: 1.into(),
                    rhs: 2.into()
                }),
                c_rhs: Box::new(Condition::Simple {
                    cond_type: ConditionType::Ne,
                    lhs: SString::Hand.into(),
                    rhs: SString::Emid.into(),
                }),
                join_type: JoinType::And
            }),
            join_type: JoinType::Or
        }
    );
}
