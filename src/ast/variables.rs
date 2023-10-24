use super::{AgentArg, IntArg, SStringArg};
use crate::Rule;
use caos_macros::ExpressionParser;

#[derive(Eq, PartialEq, Debug, Clone, ExpressionParser)]
pub enum Variable {
    #[parse(rule=Rule::variable_velx)]
    Velx,
    #[parse(rule=Rule::variable_vely)]
    Vely,
    #[parse(rule=Rule::variable_avar)]
    Avar {
        agent: Box<AgentArg>,
        index: Box<IntArg>,
    },
    #[parse(rule=Rule::variable_game)]
    Game { variable_name: Box<SStringArg> },
    #[parse(ignore)]
    Mvxx(u8),
    #[parse(ignore)]
    Ovxx(u8),
    #[parse(ignore)]
    Vaxx(u8),
    #[parse(rule=Rule::variable_p1)]
    P1,
    #[parse(rule=Rule::variable_p2)]
    P2,
}
