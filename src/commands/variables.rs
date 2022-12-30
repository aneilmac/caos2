use crate::engine::{ScriptRefMut, Variadic};
use crate::parser::CaosParseResult;
use crate::{CaosError, ErrorType, Result};
use caos_macros::{CaosParsable, CommandList, EvaluateCommand};
use nom::{
    bytes::complete::{tag_no_case, take},
    combinator::{map, map_res},
    sequence::preceded,
};

use super::{Agent, IntArg, SString};

#[derive(CaosParsable, EvaluateCommand, CommandList, Eq, PartialEq, Debug, Clone)]
#[return_type(Variadic)]
pub enum Variable {
    #[syntax]
    Velx,
    #[syntax]
    Vely,
    #[syntax]
    Avar {
        agent: Box<Agent>,
        index: Box<IntArg>,
    },
    #[syntax]
    Game { variable_name: Box<SString> },
    #[syntax(with_parser = "parse_mvxx")]
    Mvxx(u8),
    #[syntax(with_parser = "parse_ovxx")]
    Ovxx(u8),
    #[syntax(with_parser = "parse_vaxx", with_evaluator = "evaluate_vaxx")]
    Vaxx(u8),
    #[syntax(name = "_p1_")]
    P1,
    #[syntax(name = "_p2_")]
    P2,
}

fn parse_mvxx(input: &str) -> CaosParseResult<&str, Variable> {
    map(|i| parse_register("mv", i), Variable::Mvxx)(input)
}

fn parse_ovxx(input: &str) -> CaosParseResult<&str, Variable> {
    map(|i| parse_register("ov", i), Variable::Ovxx)(input)
}

fn parse_vaxx(input: &str) -> CaosParseResult<&str, Variable> {
    map(|i| parse_register("va", i), Variable::Vaxx)(input)
}

fn parse_register<'a, 'b>(prefix: &'b str, input: &'a str) -> CaosParseResult<&'a str, u8> {
    map_res(preceded(tag_no_case(prefix), take(2u32)), |s: &str| {
        s.parse::<u8>()
    })(input)
}

fn evaluate_vaxx(script: &mut ScriptRefMut<'_>, register: u8) -> Result<Variadic> {
    script
        .script_data
        .vaxx_mut()
        .get(register as usize)
        .map(|s| (*s).clone())
        .ok_or_else(|| CaosError::new(ErrorType::BadRegister))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::CaosParsable;

    #[test]
    fn test_simple_variable() {
        let (_, res) = Variable::parse_caos("_P1_").expect("Valid variable");
        assert_eq!(res, Variable::P1);
    }

    #[test]
    fn test_va_good() {
        let (_, res) = Variable::parse_caos("va05").expect("Valid variable");
        assert_eq!(res, Variable::Vaxx(5));
    }

    #[test]
    fn test_ov_lowest() {
        let (_, res) = Variable::parse_caos("ov00").expect("Valid variable");
        assert_eq!(res, Variable::Ovxx(0));
    }

    #[test]
    fn test_mv_highest() {
        let (_, res) = Variable::parse_caos("MV99").expect("Valid variable");
        assert_eq!(res, Variable::Mvxx(99));
    }

    #[test]
    fn test_bad_incomplete() {
        Variable::parse_caos("MV9 ").expect_err("Invalid variable");
    }
}
