mod script_type;
mod stack;

pub use script_type::*;

use self::stack::Stack;
use crate::Result;
use crate::commands::Command;
use crate::engine::AgentRef;
use crate::engine::Variadic;

pub struct Script {
    p1: Variadic,
    p2: Variadic,
    vaxx: Vec<Variadic>,
    ownr: Option<AgentRef>,
    targ: Option<AgentRef>,
    script_type: ScriptType,
    commands: Vec<Command>,
    stack: Stack,
}

impl Script {
    pub fn new_empty() -> Script {
        Self {
            p1: Default::default(),
            p2: Default::default(),
            vaxx:  vec![Default::default(); 100],
            ownr: None,
            targ: None,
            script_type: ScriptType::Unassigned,
            commands: vec![],
            stack: Default::default(),
        }

    }

    pub fn vaxx_mut(&mut self) -> &mut Vec<Variadic> {
        &mut self.vaxx
    }

    pub fn vaxx(&self) -> &Vec<Variadic> {
        &self.vaxx
    }

    pub fn execute_next_command(&mut self) -> Result<()> {
        let command = &self.commands[self.stack.current()?];
        todo!()
        //command.evaluate();
    }
}
