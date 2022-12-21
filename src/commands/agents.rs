use caos_macros::{CaosParsable, CommandList};
use crate::parser::CaosParsable;
use super::{Integer, SString, Variable};
use nom::combinator::map;

#[derive(CaosParsable, CommandList)]
pub enum Agent {
    #[syntax(with_parser="parse_variable")]
    Variable(Variable),
    #[syntax]
    Carr,
    #[syntax]
    From,
    #[syntax]
    Held,
    #[syntax]
    Iitt,
    #[syntax]
    Ncls {
        previous: Box<Agent>,
        family: Box<Integer>,
        genus: Box<Integer>,
        species: Box<Integer>,
    },
    #[syntax]
    Null,
    #[syntax]
    Ownr,
    #[syntax]
    Pcls {
        next: Box<Agent>,
        family: Box<Integer>,
        genus: Box<Integer>,
        species: Box<Integer>,
    },
    #[syntax]
    Pntr,
    #[syntax]
    Targ,
    #[syntax]
    Twin {
        original: Box<Agent>,
        agent_null: Box<Integer>,
    },
    #[syntax(name = "_it_")]
    It,
    #[syntax]
    Trck,
    #[syntax]
    Hhld,
    #[syntax]
    Norn,
    #[syntax]
    Agnt { unique_id: Box<Integer> },
    #[syntax]
    Tack,
    #[syntax]
    Mtoa { moniker: Box<SString> },
    #[syntax]
    Mtoc { moniker: Box<SString> },
    #[syntax]
    Hots,
}

fn parse_variable(input: &str) -> nom::IResult<&str, Agent> {
    map(Variable::parse_caos, Agent::Variable)(input)
}