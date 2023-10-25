use super::parse_expr;
use crate::ast::{DecimalArg, Variable};

fn parse_decimal_arg(content: &str) -> DecimalArg {
    parse_expr::<DecimalArg>(content)
}

#[test]
fn test_decimal_arg() {
    let p = parse_decimal_arg("MV00");
    assert_eq!(p, Variable::Mvxx(0).into());

    let p = parse_decimal_arg("3");
    assert_eq!(p, 3i32.into());

    let p = parse_decimal_arg("3.2");
    assert_eq!(p, 3.2f32.into());
}
