use crate::parser::CaosParseResult;
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_until},
    character::complete::{multispace0, multispace1},
    combinator::{consumed, opt, rest},
    sequence::preceded,
};

pub(crate) fn caos_skippable0(input: &str) -> CaosParseResult<&str, &str> {
    let (input, (consumed, ())) = consumed(|input| caos_skippable_impl(input, false))(input)?;
    Ok((input, consumed))
}

pub(crate) fn caos_skippable1(input: &str) -> CaosParseResult<&str, &str> {
    let (input, (consumed, ())) = consumed(|input| caos_skippable_impl(input, true))(input)?;
    Ok((input, consumed))
}

fn caos_skippable_impl(input: &str, requires_1: bool) -> CaosParseResult<&str, ()> {
    let (input, _) = if requires_1 {
        multispace1(input)?
    } else {
        multispace0(input)?
    };
    let (input, n) = opt(parse_comment)(input)?;
    match n {
        Some(_) => caos_skippable_impl(input, requires_1),
        None => Ok((input, ())),
    }
}

fn parse_comment(input: &str) -> CaosParseResult<&str, &str> {
    preceded(tag_no_case("*"), alt((take_until("\n"), rest)))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::Command;
    use crate::parser::CaosParsable;

    #[test]
    fn test_comment_skip() {
        let (_, c) = Command::parse_caos("brn: * hello world\ndmpb").expect("Success");
        assert_eq!(c, Command::BrnDmpb);
    }

    #[test]
    fn test_double_comment_skip() {
        let (_, c) =
            Command::parse_caos("brn: * hello world\n*this is a test\ndmpb").expect("Success");
        assert_eq!(c, Command::BrnDmpb);
    }

    #[test]
    fn test_with_space_double_comment_skip() {
        let (_, c) =
            Command::parse_caos("brn: * hello world\n *this is a test\ndmpb").expect("Success");
        assert_eq!(c, Command::BrnDmpb);
    }

    #[test]
    fn test_premature_eof() {
        let content: &str = r#"* Simple test"#;
        let (_, c) = parse_comment(content).expect("Successful parse");
        assert_eq!(c, " Simple test");
    }
}
