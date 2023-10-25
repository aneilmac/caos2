mod agent_tests;
mod decimal_tests;
mod float_tests;
mod int_tests;
mod string_tests;
mod variable_tests;

use pest::Parser;

use crate::{ast::Anything, parser::parse_expressions, parser::CaosParser, CaosError, Rule};

fn parse_expr<T>(content: &str) -> T
where
    T: TryFrom<Anything, Error = CaosError>,
{
    let p = CaosParser::parse(Rule::expressions, content).expect("Parsed");
    let p = parse_expressions(p).expect("Parsed");
    assert_eq!(p.len(), 1, "Expected one expression");
    T::try_from(p.into_iter().next().unwrap()).expect(concat!("Expected ", stringify!(T)))
}
