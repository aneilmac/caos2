use crate::parser::CaosParseResult;
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_until},
    character::complete::{multispace0, multispace1},
    combinator::{consumed, rest},
    sequence::preceded,
};

pub(crate) fn caos_skippable1(input: &str) -> CaosParseResult<&str, ()> {
    let (input, _) = alt((parse_comment, multispace1))(input)?;
    caos_skippable0(input)
}

pub(crate) fn caos_skippable0(input: &str) -> CaosParseResult<&str, ()> {
    caos_skippable_impl(input)
}

fn caos_skippable_impl(mut input: &str) -> CaosParseResult<&str, ()> {
    let mut p = consumed(alt((parse_comment, multispace0)));
    while let Ok((i, (c, _))) = p(input) {
        input = i;
        if c == "" {
            break;
        }
    }
    Ok((input, ()))
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

    #[test]
    fn test_combi() {
        let input = "\n*make a blue print agent\n";
        let (s, ()) = caos_skippable1(input).expect("Successful parse");
        assert_eq!(s, "");
    }
}
