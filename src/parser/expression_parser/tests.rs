mod agent_tests;
mod associativity_tests;
mod decimal_tests;
mod float_tests;
mod int_tests;
mod string_tests;
mod variable_tests;

use pest::Parser;

use crate::{
    ast::Anything,
    parser::{parse_expression, CaosParser},
    CaosError, Rule,
};

fn parse_expr<T>(content: &str) -> T
where
    T: TryFrom<Anything, Error = CaosError>,
{
    let mut p = CaosParser::parse(Rule::tokens, content).expect("Successful pest parse");
    let p = parse_expression(&mut p).expect("Successful expression parse");
    T::try_from(p).expect(concat!("Expected ", stringify!(T)))
}
