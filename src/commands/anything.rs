use super::{Agent, ByteString, Decimal, SString, Variable};
use crate::parser::CaosParsable;
use nom::branch::alt;
use nom::combinator::map;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Anything {
    Variable(Variable),
    String(SString),
    Decimal(Decimal),
    ByteString(ByteString),
    Agent(Agent),
}

impl CaosParsable for Anything {
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            // Important `Variable` is tested first,
            // as variable would be captured erroneously by
            // string/decimal/agent otherwise.
            map(Variable::parse_caos, Anything::Variable),
            map(SString::parse_caos, Anything::String),
            map(Decimal::parse_caos, Anything::Decimal),
            map(ByteString::parse_caos, Anything::ByteString),
            map(Agent::parse_caos, Anything::Agent),
        ))(input)
    }
}
