use super::{Agent, ByteString, Decimal, SString};
use crate::parser::CaosParsable;
use nom::branch::alt;
use nom::combinator::map;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Anything {
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
            map(SString::parse_caos, Anything::String),
            map(Decimal::parse_caos, Anything::Decimal),
            map(ByteString::parse_caos, Anything::ByteString),
            map(Agent::parse_caos, Anything::Agent),
        ))(input)
    }
}
