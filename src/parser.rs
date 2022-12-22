mod caos_parsable;
mod script_parser;

pub(crate) use caos_parsable::*;
use nom::{
    bytes::complete::{tag_no_case, take_until},
    character::complete::{multispace0, multispace1, newline},
    combinator::opt,
    sequence::delimited,
};

pub(crate) fn caos_skippable1(input: &str) -> nom::IResult<&str, ()> {
    let (input, _) = multispace1(input)?;
    let (input, _) = opt(delimited(tag_no_case("*"), take_until("\n"), newline))(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::Command;

    #[test]
    fn test_comment_skip() {
        let (_, c) = Command::parse_caos("brn: * hello world\ndmpb").expect("Success");
        assert_eq!(c, Command::BrnDmpb);
    }
}
