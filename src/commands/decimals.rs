use crate::parser::CaosParsable;

use super::{Float, Integer};
use nom::branch::alt;
use nom::combinator::map;

pub enum Decimal {
    Integer(Integer),
    Float(Float),
}

impl CaosParsable for Decimal {
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            map(Float::parse_caos, Decimal::Float), // Important this is first
            map(Integer::parse_caos, Decimal::Integer),
        ))(input)
    }
}
