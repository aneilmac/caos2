use crate::parser::CaosParseResult;
use nom::{
    bytes::complete::{tag_no_case, take_until},
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    sequence::preceded,
};

fn parse_comment(input: &str) -> CaosParseResult<&str, &str> {
    preceded(tag_no_case("*"), take_until("\n"))(input)
}

pub(crate) fn caos_skippable0(input: &str) -> CaosParseResult<&str, ()> {
    let (input, _) = multispace0(input)?;
    let (input, n) = opt(parse_comment)(input)?;
    match n {
        Some(_) => map(caos_skippable1, |_| ())(input),
        None => Ok((input, ())),
    }
}

pub(crate) fn caos_skippable1(input: &str) -> CaosParseResult<&str, ()> {
    let (input, _) = multispace1(input)?;
    let (input, n) = opt(parse_comment)(input)?;
    match n {
        Some(_) => map(caos_skippable1, |_| ())(input),
        None => Ok((input, ())),
    }
}

#[cfg(test)]
mod test {
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
}
