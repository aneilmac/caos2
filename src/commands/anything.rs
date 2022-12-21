use super::{Agent, ByteString, Float, Integer, SString};
use crate::parser::CaosParsable;
use nom::branch::alt;
use nom::combinator::map;

pub enum Anything {
    String(SString),
    Integer(Integer),
    Float(Float),
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
            map(Integer::parse_caos, Anything::Integer),
            map(Float::parse_caos, Anything::Float),
            map(ByteString::parse_caos, Anything::ByteString),
            map(Agent::parse_caos, Anything::Agent),
        ))(input)
    }
}
