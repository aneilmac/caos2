use super::{AgentArg, IntArg, SString, SStringArg, Variable};

/// Agent types represents a reference to an in-game CAOS
/// Agent.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Agent {
    Variable(Variable),
    Carr,
    From,
    Held,
    Iitt,
    Ncls {
        previous: Box<AgentArg>,
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    Null,
    Ownr,
    Pcls {
        next: Box<AgentArg>,
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    Pntr,
    Targ,
    Twin {
        original: Box<AgentArg>,
        agent_null: Box<IntArg>,
    },
    It,
    Trck,
    Hhld,
    Norn,
    Agnt { unique_id: Box<IntArg> },
    Tack,
    Mtoa { moniker: Box<SStringArg> },
    Mtoc { moniker: Box<SStringArg> },
    Hots,
    // Ports
    
    PrtFrma { input_port: Box<IntArg> },
}
