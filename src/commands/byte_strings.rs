use crate::parser::parse_integer_literal;
use crate::parser::CaosParsable;
use nom::character::complete::{char, multispace1};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq)]
pub enum ByteString {
    Raw(Vec<u8>),
}

impl CaosParsable for ByteString {
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        map(
            delimited(
                char('['),
                separated_list0(multispace1, parse_integer_literal),
                char(']'),
            ),
            |v|
            // Clamp between 0 and 255 
            ByteString::Raw(v.into_iter().map(|i| min(max(i, u8::MIN.into()), u8::MAX.into()) as u8).collect()),
        )(input)
    }
}
