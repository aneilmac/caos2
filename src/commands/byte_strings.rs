use super::LiteralInt;
use crate::engine::{EvaluateCommand, ScriptRefMut};
use crate::parser::caos_skippable1;
use crate::parser::{CaosParsable, CaosParseResult};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::tuple;
use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ByteString {
    Raw(Vec<u8>),
}

impl EvaluateCommand for ByteString {
    type ReturnType = Vec<u8>;
    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> crate::Result<Self::ReturnType> {
        match self {
            Self::Raw(v) => Ok(v.clone()),
        }
    }
}

impl CaosParsable for ByteString {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        map(
            delimited(
                tuple((tag_no_case("["), multispace0)),
                separated_list0(caos_skippable1, LiteralInt::parse_caos),
                tuple((multispace0, tag_no_case("]"))),
            ),
            |v|
            // Clamp between 0 and 255 
            ByteString::Raw(v.into_iter().map(|LiteralInt(i)| min(max(i, u8::MIN.into()), u8::MAX.into()) as u8).collect()),
        )(input)
    }
}

impl From<Vec<u8>> for ByteString {
    fn from(v: Vec<u8>) -> Self {
        ByteString::Raw(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_bit_stream() {
        let (_, res) = ByteString::parse_caos("[]").expect("Success");
        assert_eq!(res, ByteString::Raw(vec![]));
    }

    #[test]
    fn test_single_element() {
        let (_, res) = ByteString::parse_caos("[5]").expect("Success");
        assert_eq!(res, ByteString::Raw(vec![5]));
    }

    #[test]
    fn test_single_element_spaces() {
        let (_, res) = ByteString::parse_caos("[ 5 ]").expect("Success");
        assert_eq!(res, ByteString::Raw(vec![5]));
    }

    #[test]
    fn test_bit_stream() {
        let (_, res) = ByteString::parse_caos("[0 1 2 3]").expect("Success");
        assert_eq!(res, ByteString::Raw(vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_clamped_stream() {
        let (_, res) = ByteString::parse_caos("[300 300 -2]").expect("Success");
        assert_eq!(res, ByteString::Raw(vec![255, 255, 0]));
    }
}
