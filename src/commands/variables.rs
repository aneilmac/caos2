use caos_macros::{CaosParsable, CommandList};
use nom::{
    bytes::complete::{tag_no_case, take},
    combinator::{map, map_res},
    sequence::preceded,
};

use super::{Agent, Integer, SString};

#[derive(CaosParsable, CommandList, Eq, PartialEq, Debug, Clone)]
pub enum Variable {
    #[syntax]
    Velx,
    #[syntax]
    Vely,
    #[syntax]
    Avar {
        agent: Box<Agent>,
        index: Box<Integer>,
    },
    #[syntax]
    Game { variable_name: Box<SString> },
    #[syntax(with_parser = "parse_mvxx")]
    Mvxx(u8),
    #[syntax(with_parser = "parse_ovxx")]
    Ovxx(u8),
    #[syntax(with_parser = "parse_vaxx")]
    Vaxx(u8),
    #[syntax(name = "_P1_")]
    P1,
    #[syntax(name = "_P2_")]
    P2,
}

fn parse_mvxx(input: &str) -> nom::IResult<&str, Variable> {
    map(|i| parse_register("mv", i), Variable::Mvxx)(input)
}

fn parse_ovxx(input: &str) -> nom::IResult<&str, Variable> {
    map(|i| parse_register("ov", i), Variable::Ovxx)(input)
}

fn parse_vaxx(input: &str) -> nom::IResult<&str, Variable> {
    map(|i| parse_register("va", i), Variable::Vaxx)(input)
}

fn parse_register<'a, 'b>(prefix: &'b str, input: &'a str) -> nom::IResult<&'a str, u8> {
    map_res(preceded(tag_no_case(prefix), take(2u32)), |s: &str| {
        s.parse::<u8>()
    })(input)
}
