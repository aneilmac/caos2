use super::{IntArg, SString, Variable};
use caos_macros::CaosParsable;

/// Agent types represents a reference to an in-game CAOS
/// Agent.
#[derive(CaosParsable, Eq, PartialEq, Debug, Clone)]
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
    #[syntax]
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
