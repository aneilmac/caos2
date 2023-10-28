use crate::ast::{Command, Condition, ScriptDefinition};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DoIf {
    pub condition: Condition,
    pub definition: ScriptDefinition,
    pub elif_definitions: Vec<(Condition, ScriptDefinition)>,
    pub else_definition: Option<ScriptDefinition>,
}

impl DoIf {
    pub fn empty(condition: Condition) -> Self {
        Self {
            condition,
            definition: ScriptDefinition::default(),
            elif_definitions: Vec::new(),
            else_definition: None,
        }
    }

    pub fn push_elif(&mut self, condition: Condition, script_definition: ScriptDefinition) {
        self.elif_definitions.push((condition, script_definition))
    }

    pub fn set_else(&mut self, script_definition: ScriptDefinition) {
        self.else_definition = Some(script_definition);
    }

    pub fn push(&mut self, command: Command) {
        if let Some(ref mut e) = self.else_definition {
            e.push(command);
            return;
        }

        if let Some((_, e)) = self.elif_definitions.last_mut() {
            e.push(command);
            return;
        }

        self.definition.push(command);
    }
}
