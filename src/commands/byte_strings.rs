use crate::parser::CaosParsable;
use nom::character::complete::char;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::delimited;

pub enum ByteString {
    Raw(Vec<u8>),
}

impl CaosParsable for ByteString {
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        map(delimited(char('['), many0(u8), char(']')), ByteString::Raw)(input)
    }
}
