mod caos_parsable;

pub use caos_parsable::*;

use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::{anychar, char, i32};
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, preceded};
use nom::IResult;

pub fn parse_integer_literal(input: &str) -> IResult<&str, i32> {
    alt((
        // Allow parsing regular numbers
        i32,
        // Allow parsing a characters as a number
        parse_integer_char,
        // Allows parsing of binary sequences
        parse_integer_binary,
    ))(input)
}

/// Parses a character as an i32
fn parse_integer_char(input: &str) -> IResult<&str, i32> {
    map(delimited(char('\''), anychar, char('\'')), |c| c as i32)(input)
}

/// Parses a %XXXXX as an i32
fn parse_integer_binary(input: &str) -> IResult<&str, i32> {
    map_res(
        preceded(char('%'), take_while1(|c| c == '0' || c == '1')),
        |b| i32::from_str_radix(b, 2),
    )(input)
}

#[cfg(test)]
mod test {
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

    #[test]
    fn test_integer() {
        let (_, res) = parse_integer_literal("%11").expect("Good binary");
        assert_eq!(res, 3);

        let (_, res) = parse_integer_literal("'A'").expect("Good binary");
        assert_eq!(res, 65);

        let (_, res) = parse_integer_literal("32").expect("Good binary");
        assert_eq!(res, 32);
    }
}
