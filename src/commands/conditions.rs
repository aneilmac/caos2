use crate::{
    engine::{EvaluateCommand, ScriptRefMut},
    parser::caos_skippable1,
};
use nom::{branch::alt, bytes::complete::tag_no_case, combinator::map};

use super::Anything;
use crate::parser::{CaosParsable, CaosParseResult};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Condition {
    Simple {
        cond_type: ConditionType,
        lhs: Anything,
        rhs: Anything,
    },
    Combination {
        c_lhs: Box<Condition>,
        c_rhs: Box<Condition>,
        join_type: JoinType,
    },
}

impl CaosParsable for Condition {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        // CAOS conditions are very primitive. They are evaluated from left to right,
        // with no parentheses available.
        let (input, lhs) = parse_condition_simple(input)?;
        let res: CaosParseResult<&str, (Condition, JoinType)> = (|input| {
            let (input, _) = caos_skippable1(input)?;
            let (input, join_type) = JoinType::parse_caos(input)?;
            let (input, _) = caos_skippable1(input)?;
            let (input, rhs) = CaosParsable::parse_caos(input)?;
            Ok((input, (rhs, join_type)))
        })(input);

        match res {
            Err(..) => Ok((input, lhs)),
            Ok((rhs_input, (rhs, join_type))) => Ok((
                rhs_input,
                Condition::Combination {
                    c_lhs: Box::new(lhs),
                    c_rhs: Box::new(rhs),
                    join_type: join_type,
                },
            )),
        }
    }
}

fn parse_condition_simple(input: &str) -> CaosParseResult<&str, Condition> {
    let (input, lhs) = Anything::parse_caos(input)?;
    let (input, _) = caos_skippable1(input)?;
    let (input, cond_type) = ConditionType::parse_caos(input)?;
    let (input, _) = caos_skippable1(input)?;
    let (input, rhs) = Anything::parse_caos(input)?;
    Ok((
        input,
        Condition::Simple {
            cond_type,
            lhs,
            rhs,
        },
    ))
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum JoinType {
    And,
    Or,
}

impl CaosParsable for JoinType {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            map(tag_no_case("and"), |_| JoinType::And),
            map(tag_no_case("or"), |_| JoinType::Or),
        ))(input)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ConditionType {
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
}

impl CaosParsable for ConditionType {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            map(alt((tag_no_case("="), tag_no_case("eq"))), |_| {
                ConditionType::Eq
            }),
            map(alt((tag_no_case(">="), tag_no_case("ge"))), |_| {
                ConditionType::Ge
            }),
            map(alt((tag_no_case(">"), tag_no_case("gt"))), |_| {
                ConditionType::Gt
            }),
            map(alt((tag_no_case("<="), tag_no_case("le"))), |_| {
                ConditionType::Le
            }),
            map(alt((tag_no_case("<>"), tag_no_case("ne"))), |_| {
                ConditionType::Ne
            }),
            map(alt((tag_no_case("<"), tag_no_case("lt"))), |_| {
                ConditionType::Lt
            }),
        ))(input)
    }
}

impl EvaluateCommand for Condition {
    type ReturnType = bool;

    fn evaluate(&self, script: &mut ScriptRefMut<'_>) -> crate::Result<Self::ReturnType> {
        match self {
            Condition::Simple {
                cond_type,
                lhs,
                rhs,
            } => {
                let lhs = lhs.evaluate(script)?;
                let rhs = rhs.evaluate(script)?;
                Ok(match cond_type {
                    ConditionType::Eq => lhs.eq(&rhs)?,
                    ConditionType::Ne => !lhs.eq(&rhs)?,
                    ConditionType::Ge => lhs.partial_cmp(&rhs)?.map(|f| f.is_ge()).unwrap_or(false),
                    ConditionType::Gt => lhs.partial_cmp(&rhs)?.map(|f| f.is_gt()).unwrap_or(false),
                    ConditionType::Le => lhs.partial_cmp(&rhs)?.map(|f| f.is_le()).unwrap_or(false),
                    ConditionType::Lt => lhs.partial_cmp(&rhs)?.map(|f| f.is_lt()).unwrap_or(false),
                })
            }
            Condition::Combination {
                c_lhs,
                c_rhs,
                join_type,
            } => {
                // According to the Creatures Wiki this conditional does **not** short
                // circuit in the original CAOS implementation --
                // thus we will not short circuit either until it can be proven it is an optimization
                // that leads to incorrect behaviour.
                let c_lhs = c_lhs.evaluate(script)?;
                let c_rhs = c_rhs.evaluate(script)?;
                Ok(match join_type {
                    JoinType::And => c_lhs && c_rhs,
                    JoinType::Or => c_lhs || c_rhs,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::{Command, Float, IntArg, Integer, Variable};

    use super::*;

    #[test]
    fn test_joins() {
        let (_, res) = JoinType::parse_caos("and").expect("Valid join");
        assert_eq!(res, JoinType::And);

        let (_, res) = JoinType::parse_caos("OR").expect("Valid join");
        assert_eq!(res, JoinType::Or);
    }

    #[test]
    fn test_conditions() {
        let (_, res) = ConditionType::parse_caos("=").expect("Valid condition");
        assert_eq!(res, ConditionType::Eq);

        let (_, res) = ConditionType::parse_caos("EQ").expect("Valid condition");
        assert_eq!(res, ConditionType::Eq);

        let (_, res) = ConditionType::parse_caos("<>").expect("Valid condition");
        assert_eq!(res, ConditionType::Ne);

        let (_, res) = ConditionType::parse_caos("NE").expect("Valid condition");
        assert_eq!(res, ConditionType::Ne);

        let (_, res) = ConditionType::parse_caos("GE").expect("Valid condition");
        assert_eq!(res, ConditionType::Ge);

        let (_, res) = ConditionType::parse_caos(">=").expect("Valid condition");
        assert_eq!(res, ConditionType::Ge);

        let (_, res) = ConditionType::parse_caos(">").expect("Valid condition");
        assert_eq!(res, ConditionType::Gt);

        let (_, res) = ConditionType::parse_caos("GT").expect("Valid condition");
        assert_eq!(res, ConditionType::Gt);

        let (_, res) = ConditionType::parse_caos("<=").expect("Valid condition");
        assert_eq!(res, ConditionType::Le);

        let (_, res) = ConditionType::parse_caos("LE").expect("Valid condition");
        assert_eq!(res, ConditionType::Le);

        let (_, res) = ConditionType::parse_caos("<").expect("Valid condition");
        assert_eq!(res, ConditionType::Lt);

        let (_, res) = ConditionType::parse_caos("LT").expect("Valid condition");
        assert_eq!(res, ConditionType::Lt);
    }

    #[test]
    fn test_simple_condition_lt_1() {
        use crate::commands::Decimal;
        let (_, res) = parse_condition_simple("3.13 < 5").expect("Valid condition");
        assert_eq!(
            res,
            Condition::Simple {
                cond_type: ConditionType::Lt,
                lhs: Anything::Decimal(Decimal::Float(3.13.into())),
                rhs: Anything::Decimal(Decimal::Integer(5.into()))
            }
        );
    }

    #[test]
    fn test_simple_condition_lt_2() {
        let (_, res) = parse_condition_simple("VA34 LT VA00").expect("Valid condition");
        assert_eq!(
            res,
            Condition::Simple {
                cond_type: ConditionType::Lt,
                lhs: Anything::Variable(Variable::Vaxx(34)),
                rhs: Anything::Variable(Variable::Vaxx(0))
            }
        );
    }

    #[test]
    fn test_condition_parse() {
        let (_, res) = Condition::parse_caos("VA34 LT VA00").expect("Valid condition");
        assert_eq!(
            res,
            Condition::Simple {
                cond_type: ConditionType::Lt,
                lhs: Anything::Variable(Variable::Vaxx(34)),
                rhs: Anything::Variable(Variable::Vaxx(0))
            }
        );
    }

    #[test]
    fn test_condition_as_combo() {
        use crate::commands::Decimal;
        let (_, res) = Condition::parse_caos("VA34 LT VA00 AND 3.13 < 5").expect("Valid condition");
        assert_eq!(
            res,
            Condition::Combination {
                c_lhs: Box::new(Condition::Simple {
                    cond_type: ConditionType::Lt,
                    lhs: Anything::Variable(Variable::Vaxx(34)),
                    rhs: Anything::Variable(Variable::Vaxx(0))
                }),
                c_rhs: Box::new(Condition::Simple {
                    cond_type: ConditionType::Lt,
                    lhs: Anything::Decimal(Decimal::Float(3.13.into())),
                    rhs: Anything::Decimal(Decimal::Integer(5.into()))
                }),
                join_type: JoinType::And
            }
        );
    }

    #[test]
    fn test_triple_condition_combo() {
        use crate::commands::Decimal;
        let (_, res) = Condition::parse_caos("VA34 LT VA00 AND 3.13 < 5 OR \"hello\" <> \"world\"")
            .expect("Valid condition");
        assert_eq!(
            res,
            Condition::Combination {
                c_lhs: Box::new(Condition::Simple {
                    cond_type: ConditionType::Lt,
                    lhs: Anything::Variable(Variable::Vaxx(34)),
                    rhs: Anything::Variable(Variable::Vaxx(0))
                }),
                c_rhs: Box::new(Condition::Combination {
                    c_lhs: Box::new(Condition::Simple {
                        cond_type: ConditionType::Lt,
                        lhs: Anything::Decimal(Decimal::Float(3.13.into())),
                        rhs: Anything::Decimal(Decimal::Integer(5.into()))
                    }),
                    c_rhs: Box::new(Condition::Simple {
                        cond_type: ConditionType::Ne,
                        lhs: Anything::String("hello".to_owned().into()),
                        rhs: Anything::String("world".to_owned().into())
                    }),
                    join_type: JoinType::Or
                }),
                join_type: JoinType::And
            }
        );
    }

    #[test]
    fn test_complex_condition() {
        let (_, res) =
            Command::parse_caos("doif aslp ne 0 or dead eq 1 or uncs eq 1 or loci 1 1 4 9 ne 0.0")
                .expect("Successful parse");
        assert_eq!(
            res,
            Command::Doif {
                condition: Condition::Combination {
                    c_lhs: Box::new(Condition::Simple {
                        cond_type: ConditionType::Ne,
                        lhs: Integer::Aslp.into(),
                        rhs: Integer::from(0).into(),
                    }),
                    c_rhs: Box::new(Condition::Combination {
                        c_lhs: Box::new(Condition::Simple {
                            cond_type: ConditionType::Eq,
                            lhs: Integer::Dead.into(),
                            rhs: Integer::from(1).into(),
                        }),
                        c_rhs: Box::new(Condition::Combination {
                            c_lhs: Box::new(Condition::Simple {
                                cond_type: ConditionType::Eq,
                                lhs: Integer::Uncs.into(),
                                rhs: Integer::from(1).into()
                            }),
                            c_rhs: Box::new(Condition::Simple {
                                cond_type: ConditionType::Ne,
                                lhs: Float::Loci {
                                    r#type: Box::new(IntArg::from_primary(1.into())),
                                    organ: Box::new(IntArg::from_primary(1.into())),
                                    tissue: Box::new(IntArg::from_primary(4.into())),
                                    id: Box::new(IntArg::from_primary(9.into())),
                                }
                                .into(),
                                rhs: Float::from(0.0).into()
                            }),
                            join_type: JoinType::Or,
                        }),
                        join_type: JoinType::Or,
                    }),
                    join_type: JoinType::Or
                }
            }
        );
    }
}
