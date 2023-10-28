use super::parse_expr;
use crate::{
    ast::{Agent, Variable},
    parser::CaosParser,
    Rule,
};
use pest::Parser;

fn parse_variable(content: &str) -> Variable {
    parse_expr::<Variable>(content)
}

#[test]
fn test_variable_mvxx() {
    let p = parse_variable("MV00");
    assert_eq!(p, Variable::Mvxx(0));

    let p = parse_variable("MV32");
    assert_eq!(p, Variable::Mvxx(32));

    let p = parse_variable("MV99");
    assert_eq!(p, Variable::Mvxx(99));
}

#[test]
fn test_variable_mvxx_fail() {
    CaosParser::parse(Rule::token, "MV3").expect_err("Parse failure");
    CaosParser::parse(Rule::token, "MVFF").expect_err("Parse failure");
}

#[test]
fn test_variable_ovxx() {
    let p = parse_variable("OV00");
    assert_eq!(p, Variable::Ovxx(0));

    let p = parse_variable("OV32");
    assert_eq!(p, Variable::Ovxx(32));

    let p = parse_variable("OV99");
    assert_eq!(p, Variable::Ovxx(99));
}

#[test]
fn test_variable_ovxx_fail() {
    CaosParser::parse(Rule::token, "OV3").expect_err("Parse failure");
    CaosParser::parse(Rule::token, "OVFF").expect_err("Parse failure");
}

#[test]
fn test_variable_vaxx() {
    let p = parse_variable("VA00");
    assert_eq!(p, Variable::Vaxx(0));

    let p = parse_variable("VA32");
    assert_eq!(p, Variable::Vaxx(32));

    let p = parse_variable("VA99");
    assert_eq!(p, Variable::Vaxx(99));
}

#[test]
fn test_variable_vaxx_fail() {
    CaosParser::parse(Rule::token, "VA3").expect_err("Parse failure");
    CaosParser::parse(Rule::token, "VAFF").expect_err("Parse failure");
}

#[test]
fn test_variable_game() {
    let p = parse_variable(r#"GAME "Hello""#);
    assert_eq!(
        p,
        Variable::Game {
            variable_name: Box::new(String::from("Hello").into())
        }
    );
}

#[test]
fn test_variable_avar() {
    let p = parse_variable("AVAR CARR 34");
    assert_eq!(
        p,
        Variable::Avar {
            agent: Box::new(Agent::Carr.into()),
            index: Box::new(34.into())
        }
    );
}
