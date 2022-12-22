use crate::parser::caos_skippable1;
use nom::{branch::alt, bytes::complete::tag_no_case, combinator::map};

use super::Anything;
use crate::parser::CaosParsable;

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
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        // CAOS conditions are very primitive. They are evaluated from left to right,
        // with no parentheses available.
        let (input, lhs) = parse_condition_simple(input)?;
        let res: nom::IResult<&str, (Condition, JoinType)> = (|input| {
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

fn parse_condition_simple(input: &str) -> nom::IResult<&str, Condition> {
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
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
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
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
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

#[cfg(test)]
mod tests {
    use crate::commands::Variable;

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
}
