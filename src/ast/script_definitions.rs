use crate::ast::Command;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EventScriptDefinition {
    pub definition: ScriptDefinition,
    pub family: i32,
    pub genus: i32,
    pub species: i32,
    pub script_number: i32,
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct ScriptDefinition {
    pub commands: Vec<Command>,
}

impl ScriptDefinition {
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Script {
    Install(ScriptDefinition),
    Removal(ScriptDefinition),
    Event(EventScriptDefinition),
}
