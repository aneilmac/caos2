use pest::Parser;

use super::parse_expr;
use crate::{
    ast::{IntArg, Integer},
    parse_expression,
    parser::CaosParser,
    Rule,
};

#[test]
fn test_partial_fail() {
    let mut p = CaosParser::parse(Rule::tokens, "MUTE 1").expect("Parsed");
    parse_expression(&mut p).expect_err("Expected to fail with too few arguments");
}

#[test]
fn test_empty() {
    let mut p = CaosParser::parse(Rule::tokens, "").expect("Parsed");
    parse_expression(&mut p).expect_err("Empty stream");
}

#[test]
fn test_right_assoc() {
    let p = parse_expr::<IntArg>("MUTE 1 MUTE 1 2");
    assert_eq!(
        p,
        Integer::Mute {
            and_mask: Box::new(1.into()),
            eor_mask: Box::new(
                Integer::Mute {
                    and_mask: Box::new(1.into()),
                    eor_mask: Box::new(2.into())
                }
                .into()
            )
        }
        .into()
    );
}

#[test]
fn test_right_assoc_nested() {
    let p = parse_expr::<IntArg>("MUTE 1 MUTE 1 MUTE 3 4");
    assert_eq!(
        p,
        Integer::Mute {
            and_mask: Box::new(1.into()),
            eor_mask: Box::new(
                Integer::Mute {
                    and_mask: Box::new(1.into()),
                    eor_mask: Box::new(
                        Integer::Mute {
                            and_mask: Box::new(3.into()),
                            eor_mask: Box::new(4.into())
                        }
                        .into()
                    )
                }
                .into()
            )
        }
        .into()
    );
}

#[test]
fn test_left_assoc() {
    let p = parse_expr::<IntArg>("MUTE MUTE 1 2 1");
    assert_eq!(
        p,
        Integer::Mute {
            and_mask: Box::new(
                Integer::Mute {
                    and_mask: Box::new(1.into()),
                    eor_mask: Box::new(2.into())
                }
                .into()
            ),
            eor_mask: Box::new(1.into())
        }
        .into()
    );
}

#[test]
fn test_left_assoc_nested() {
    let p = parse_expr::<IntArg>("MUTE MUTE MUTE 1 2 3 4");
    assert_eq!(
        p,
        Integer::Mute {
            and_mask: Box::new(
                Integer::Mute {
                    and_mask: Box::new(
                        Integer::Mute {
                            and_mask: Box::new(1.into()),
                            eor_mask: Box::new(2.into())
                        }
                        .into()
                    ),
                    eor_mask: Box::new(3.into())
                }
                .into()
            ),
            eor_mask: Box::new(4.into())
        }
        .into()
    );
}
