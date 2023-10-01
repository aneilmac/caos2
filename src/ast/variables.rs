use super::{Agent, IntArg, SString};
use crate::{CaosError, ErrorType, Result};
use caos_macros::CaosParsable;

#[derive(CaosParsable, Eq, PartialEq, Debug, Clone)]
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
    #[syntax(with_parser = "parse_vaxx")]
    Vaxx(u8),
    #[syntax(name = "_p1_")]
    P1,
    #[syntax(name = "_p2_")]
    P2,
}
