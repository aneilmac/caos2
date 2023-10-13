use super::{AgentArg, IntArg, SStringArg};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Variable {
    Velx,
    Vely,
    Avar {
        agent: Box<AgentArg>,
        index: Box<IntArg>,
    },
    Game { variable_name: Box<SStringArg> },
    Mvxx(u8),
    Ovxx(u8),
    Vaxx(u8),
    P1,
    P2,
}
