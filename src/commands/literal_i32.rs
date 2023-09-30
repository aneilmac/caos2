use crate::engine::{EvaluateCommand, ScriptRefMut};
use crate::parser::{CaosParsable, CaosParseResult};
use nom::bytes::complete::take_while1;
use nom::character::complete::{anychar, char, i32 as i32p};
use nom::combinator::{map, map_res};
use nom::error::{ErrorKind, ParseError, VerboseError};
use nom::sequence::{delimited, preceded};

/// Represents an integer that can only be parsed as a numeric literal.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct LiteralInt(pub i32);

impl EvaluateCommand for LiteralInt {
    type ReturnType = i32;

    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> crate::Result<Self::ReturnType> {
        Ok(self.0)
    }
}

impl CaosParsable for LiteralInt {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        let res: CaosParseResult<&str, i32> = i32p(input);
        if let Ok((input, res)) = res {
            return Ok((input, LiteralInt(res)));
        }

        let res: CaosParseResult<&str, i32> = parse_integer_char(input);
        if let Ok((input, res)) = res {
            return Ok((input, LiteralInt(res)));
        }

        let res: CaosParseResult<&str, i32> = parse_integer_binary(input);
        if let Ok((input, res)) = res {
            return Ok((input, LiteralInt(res)));
        }

        Err(nom::Err::Error(VerboseError::from_error_kind(
            input,
            ErrorKind::Alt,
        )))
    }
}

impl From<i32> for LiteralInt {
    fn from(i: i32) -> Self {
        LiteralInt(i)
    }
}

impl From<LiteralInt> for i32 {
    fn from(i: LiteralInt) -> Self {
        i.0
    }
}

/// Parses a character as an i32
fn parse_integer_char(input: &str) -> CaosParseResult<&str, i32> {
    map(delimited(char('\''), anychar, char('\'')), |c| c as i32)(input)
}

/// Parses a %XXXXX as an i32
fn parse_integer_binary(input: &str) -> CaosParseResult<&str, i32> {
    map_res(
        preceded(char('%'), take_while1(|c| c == '0' || c == '1')),
        |b| i32::from_str_radix(b, 2),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_binary() {
        parse_integer_binary("%").expect_err("Bad binary");
        parse_integer_binary("%3").expect_err("Bad binary");
        parse_integer_binary("0101").expect_err("Bad binary");
    }

    #[test]
    fn test_binary_0() {
        let (_, res) = parse_integer_binary("%0").expect("Good binary");
        assert_eq!(res, 0);
    }

    #[test]
    fn test_binary_3() {
        let (_, res) = parse_integer_binary("%11").expect("Good binary");
        assert_eq!(res, 3);
    }

    #[test]
    fn test_char_good() {
        let (_, res) = parse_integer_char("'N'").expect("Good binary");
        assert_eq!(res, 78);
    }

    #[test]
    fn test_char_bad() {
        parse_integer_char("N").expect_err("Bad binary");
        parse_integer_char("''").expect_err("Bad binary");
        parse_integer_char("'Q").expect_err("Bad binary");
    }
}
