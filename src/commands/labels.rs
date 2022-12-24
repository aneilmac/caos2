use crate::parser::{CaosParsable, CaosParseResult};
use nom::{bytes::complete::is_a, character::complete::alpha1, combinator::opt};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Label(String);

impl CaosParsable for Label {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, first_chars) = alpha1(input)?;
        let (input, remainder) = opt(is_a(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_",
        ))(input)?;
        Ok((
            input,
            Label(format!("{}{}", first_chars, remainder.unwrap_or(""))),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let (_, res) = Label::parse_caos("foo").expect("Successful parse");
        assert_eq!(Label(String::from("foo")), res);
    }

    #[test]
    fn test_simple_2() {
        let (_, res) = Label::parse_caos("foo_").expect("Successful parse");
        assert_eq!(Label(String::from("foo_")), res);
    }

    #[test]
    fn test_bad_label() {
        Label::parse_caos("2foo_").expect_err("Should not start with number.");
    }
}
