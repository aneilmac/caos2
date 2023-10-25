use pest::Parser;

use super::parse_exprs;
use crate::{
    ast::{Float, Integer, Variable},
    parse_expressions,
    parser::CaosParser,
    Rule,
};

#[test]
fn test_partial_fail() {
    let p = CaosParser::parse(Rule::expressions, "MUTE 1").expect("Parsed");
    parse_expressions(p).expect_err("Expected to fail with too few arguments");
}

#[test]
fn test_good_then_partial_fail() {
    let p = CaosParser::parse(Rule::expressions, "MTHX MUTE 1").expect("Parsed");
    parse_expressions(p).expect_err("Expected to fail with too few arguments");
}

#[test]
fn test_empty() {
    let p = parse_exprs("");
    assert_eq!(p, vec![]);
}

#[test]
fn test_single() {
    let p = parse_exprs("MTHX");
    assert_eq!(p, vec![Float::Mthx.into()]);
}

#[test]
fn test_many() {
    let p = parse_exprs("MTHX MV32 ATTR");
    assert_eq!(
        p,
        vec![
            Float::Mthx.into(),
            Variable::Mvxx(32).into(),
            Integer::Attr.into()
        ]
    );
}

#[test]
fn test_right_assoc() {
    let p = parse_exprs("MUTE 1 MUTE 1 2");
    assert_eq!(
        p,
        vec![Integer::Mute {
            and_mask: Box::new(1.into()),
            eor_mask: Box::new(
                Integer::Mute {
                    and_mask: Box::new(1.into()),
                    eor_mask: Box::new(2.into())
                }
                .into()
            )
        }
        .into()]
    );
}

#[test]
fn test_right_assoc_many() {
    let p = parse_exprs("MUTE 1 MUTE 1 2 MUTE 1 MUTE 1 2");
    assert_eq!(
        p,
        vec![
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
            .into(),
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
        ]
    );
}

#[test]
fn test_right_assoc_nested() {
    let p = parse_exprs("MUTE 1 MUTE 1 MUTE 3 4");
    assert_eq!(
        p,
        vec![Integer::Mute {
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
        .into()]
    );
}

#[test]
fn test_left_assoc() {
    let p = parse_exprs("MUTE MUTE 1 2 1");
    assert_eq!(
        p,
        vec![Integer::Mute {
            and_mask: Box::new(
                Integer::Mute {
                    and_mask: Box::new(1.into()),
                    eor_mask: Box::new(2.into())
                }
                .into()
            ),
            eor_mask: Box::new(1.into())
        }
        .into()]
    );
}

#[test]
fn test_left_assoc_many() {
    let p = parse_exprs("MUTE MUTE 1 2 1 MUTE MUTE 1 2 1");
    assert_eq!(
        p,
        vec![
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
            .into(),
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
        ]
    );
}

#[test]
fn test_left_assoc_nested() {
    let p = parse_exprs("MUTE MUTE MUTE 1 2 3 4");
    assert_eq!(
        p,
        vec![Integer::Mute {
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
        .into()]
    );
}
