use super::{AgentArg, IntArg, SStringArg};
use crate::Rule;
use caos_macros::ExpressionParser;

/// Agent types represents a reference to an in-game CAOS
/// Agent.
#[derive(Eq, PartialEq, Debug, Clone, ExpressionParser)]
pub enum Agent {
    #[parse(rule=Rule::agent_carr)]
    Carr,
    #[parse(rule=Rule::agent_from)]
    From,
    #[parse(rule=Rule::agent_held)]
    Held,
    #[parse(rule=Rule::agent_iitt)]
    Iitt,
    #[parse(rule=Rule::agent_ncls)]
    Ncls {
        previous: Box<AgentArg>,
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[parse(rule=Rule::agent_null)]
    Null,
    #[parse(rule=Rule::agent_ownr)]
    Ownr,
    #[parse(rule=Rule::agent_pcls)]
    Pcls {
        next: Box<AgentArg>,
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[parse(rule=Rule::agent_pntr)]
    Pntr,
    #[parse(rule=Rule::agent_targ)]
    Targ,
    #[parse(rule=Rule::agent_twin)]
    Twin {
        original: Box<AgentArg>,
        agent_null: Box<IntArg>,
    },
    #[parse(rule=Rule::agent_it)]
    It,
    #[parse(rule=Rule::agent_trck)]
    Trck,
    #[parse(rule=Rule::agent_hhld)]
    Hhld,
    #[parse(rule=Rule::agent_norn)]
    Norn,
    #[parse(rule=Rule::agent_agnt)]
    Agnt { unique_id: Box<IntArg> },
    #[parse(rule=Rule::agent_tack)]
    Tack,
    #[parse(rule=Rule::agent_mtoa)]
    Mtoa { moniker: Box<SStringArg> },
    #[parse(rule=Rule::agent_mtoc)]
    Mtoc { moniker: Box<SStringArg> },
    #[parse(rule=Rule::agent_hots)]
    Hots,
    // Ports
    #[parse(rule=Rule::agent_prt_frma)]
    PrtFrma { input_port: Box<IntArg> },
}
