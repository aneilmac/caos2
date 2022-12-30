mod flow;
mod script_type;
mod stack;

pub use script_type::*;

use self::stack::Stack;
use crate::engine::AgentRef;
use crate::engine::Variadic;
use crate::Result;
use crate::parser::ScriptDefinition;

//#[derive(Clone)]
pub(crate) struct ScriptRefMut<'a> {
    pub script_data: &'a mut ScriptData,
    pub definition: &'a ScriptDefinition,
    pub stack: &'a mut Stack,
}

impl<'a> ScriptRefMut<'a> {
    pub(crate) fn new(script: &'a mut Script) -> Self {
        Self {
            script_data: &mut script.data,
            definition: &script.definition,
            stack: &mut script.stack,
        }
    }
}

pub struct ScriptData {
    p1: Variadic,
    p2: Variadic,
    vaxx: Vec<Variadic>,
    ownr: Option<AgentRef>,
    targ: Option<AgentRef>,
}

impl ScriptData {
    fn new_empty() -> Self {
        Self {
            p1: Default::default(),
            p2: Default::default(),
            vaxx: vec![Default::default(); 100],
            ownr: None,
            targ: None,
        }
    }

    pub fn targ_mut(&mut self) -> &mut Option<AgentRef> {
        &mut self.targ
    }

    pub fn targ(&self) -> &Option<AgentRef> {
        &self.targ
    }

    pub fn vaxx_mut(&mut self) -> &mut Vec<Variadic> {
        &mut self.vaxx
    }

    pub fn vaxx(&self) -> &Vec<Variadic> {
        &self.vaxx
    }
}

pub struct Script {
    data: ScriptData,
    script_type: ScriptType,
    definition: ScriptDefinition,
    stack: Stack,
}

impl Script {
    pub fn new_empty() -> Script {
        Self {
            data: ScriptData::new_empty(),
            script_type: ScriptType::Unassigned,
            definition: Default::default(),
            stack: Default::default(),
        }
    }

    pub fn execute_next_command(&mut self) -> Result<()> {
        use crate::engine::EvaluateCommand;
        let index = self.stack.current()?;

        let com = self.definition.commands.get(index);
        let script_data = &mut self.data;
        let stack = &mut self.stack;
        let mut script_ref = ScriptRefMut {
            script_data,
            stack,
            definition: &self.definition,
        };

        com.map(move |c| {
            c.evaluate(&mut script_ref)
                .and_then(|_| script_ref.stack.next())
        })
        .unwrap_or_else(|| Ok(()))
    }
}
