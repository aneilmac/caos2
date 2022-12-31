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

pub(crate) use evaluate_command::*;

use std::collections::HashMap;

pub struct Engine {
    game_variables: HashMap<String, Variadic>,
    agents: Vec<Agent>,
}

impl Engine {
    //pub fn inject()

    pub fn tick(&mut self) {}
}

// Scripts send messages, which may trigger on other scripts
// Scripts have data which is mutable, #
// A script (running) may trigger a command,
//
// Commands mutate (specific) variables
// Commands mutate script data
// Commands mutate agent data
// Commands mutate world data
// Commands change what command is run next - and in-fact do
//
// Commands are run in sequence
//
// Commands may generate new scripts
//
// Deleting an agent deletes the associated scripts
//
// Agents live as long as the world
// Scripts live as long as the agents
// Commands live as long as the scripts

// Commands should only run 1 at a time. Commands
