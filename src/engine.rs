mod agent;
mod agent_ref;
mod evaluate_command;
mod message_type;
mod script;
mod stimulus_type;
mod variadic;

pub use agent::*;
pub use agent_ref::*;
pub use message_type::*;
pub use script::*;
pub use stimulus_type::*;
pub use variadic::*;

pub(in crate) use evaluate_command::*;

use std::collections::HashMap;

pub struct Engine {
    game_variables: HashMap<String, Variadic>,
    agents: Vec<Agent>,
}

impl Engine {
    //pub fn inject()

    pub fn tick(&mut self) {}
}
