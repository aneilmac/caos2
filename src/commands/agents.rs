mod evaluators;

use evaluators::*;
use super::{IntArg, SString, Variable};
use crate::parser::{CaosParsable, CaosParseResult};
use caos_macros::{CaosParsable, CommandList, EvaluateCommand};
use nom::combinator::map;

/// Agent types represents a reference to an in-game CAOS
/// Agent.
#[derive(CaosParsable, EvaluateCommand, CommandList, Eq, PartialEq, Debug, Clone)]
#[return_type(crate::engine::AgentRef)]
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
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[syntax]
    Null,
    #[syntax]
    Ownr,
    #[syntax]
    Pcls {
        next: Box<Agent>,
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[syntax]
    Pntr,
    #[syntax(with_evaluator="eval_targ")]
    Targ,
    #[syntax]
    Twin {
        original: Box<Agent>,
        agent_null: Box<IntArg>,
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
    Agnt { unique_id: Box<IntArg> },
    #[syntax]
    Tack,
    #[syntax]
    Mtoa { moniker: Box<SString> },
    #[syntax]
    Mtoc { moniker: Box<SString> },
    #[syntax]
    Hots,
    // Ports
    #[syntax(name = "prt: frma")]
    PrtFrma { input_port: Box<IntArg> },
}

fn parse_variable(input: &str) -> CaosParseResult<&str, Agent> {
    map(Variable::parse_caos, Agent::Variable)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::Integer;

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
                agent_null: Box::new(IntArg::from(Integer::from(2)))
            }
        );
    }
}
