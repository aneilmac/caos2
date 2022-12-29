mod script_type;
mod stack;

pub use script_type::*;

use self::stack::Stack;
use crate::commands::Command;
use crate::engine::AgentRef;
use crate::engine::Variadic;

pub struct Script {
    p1: Variadic,
    p2: Variadic,
    vaxx: [Variadic; 100],
    ownr: Option<AgentRef>,
    targ: Option<AgentRef>,
    script_type: ScriptType,
    commands: Vec<Command>,
    command_index: usize,
    stack: Stack,
}

impl Script {
    pub fn execute_next_command(&mut self) {
        let command = &self.commands[self.command_index];
        todo!()
        //command.evaluate();
    }
}
