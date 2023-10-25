mod agent_tests;
mod associativity_tests;
mod decimal_tests;
mod float_tests;
mod int_tests;
mod string_tests;
mod variable_tests;

use crate::{
    ast::Anything,
    parser::{parse_expressions, CaosParser},
    CaosError, Rule,
};
use pest::Parser;

fn parse_exprs(content: &str) -> Vec<Anything> {
    let p = CaosParser::parse(Rule::expressions, content).expect("Parsed");
    parse_expressions(p).expect("Parsed")
}

fn parse_expr<T>(content: &str) -> T
where
    T: TryFrom<Anything, Error = CaosError>,
{
    let p = parse_exprs(content);
    assert_eq!(p.len(), 1, "Expected one expression");
    T::try_from(p.into_iter().next().unwrap()).expect(concat!("Expected ", stringify!(T)))
}
