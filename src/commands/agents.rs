use super::{Integer, SString, Variable};
use crate::parser::CaosParsable;
use caos_macros::{CaosParsable, CommandList};
use nom::combinator::map;

/// Agent types represents a reference to an in-game CAOS
/// Agent.
#[derive(CaosParsable, CommandList, Eq, PartialEq, Debug, Clone)]
pub enum Agent {
    #[syntax(with_parser = "parse_variable")]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_agent() {
        let (_, agent) = Agent::parse_caos("hots").expect("Valid Agent");
        assert_eq!(agent, Agent::Hots);
    }

    #[test]
    fn test_compound_agent() {
        let (_, agent) = Agent::parse_caos("twin _it_ %10").expect("Valid Agent");
        assert_eq!(
            agent,
            Agent::Twin {
                original: Box::new(Agent::It),
                agent_null: Box::new(2.into())
            }
        );
    }
}
