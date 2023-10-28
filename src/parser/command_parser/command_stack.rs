use super::{CommandThunk, Control};

use crate::{
    ast::{Command, ScriptDefinition},
    CaosError, Rule,
};
use std::vec::Vec;

pub struct CommandStack {
    pub commands: Vec<Command>,
    pub partials: Vec<Control>,
}

impl CommandStack {
    /// Creates a new empty stack
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            partials: Vec::new(),
        }
    }

    pub fn to_commands(self) -> Result<Vec<Command>, CaosError> {
        if let Some(_e) = self.partials.into_iter().last() {
            return Err(CaosError::new_end_of_stream());
        }
        Ok(self.commands)
    }

    fn push_command(&mut self, c: Command) {
        match self.partials.last_mut() {
            None => self.commands.push(c),
            Some(control) => control.push(c),
        }
    }

    pub fn push(&mut self, thunk: CommandThunk<'_>) -> Result<(), CaosError> {
        match thunk {
            CommandThunk::Partial(p) => {
                if p.is_ready() {
                    let c = p.complete()?;
                    self.push_command(c);
                    Ok(())
                } else {
                    Err(CaosError::new_parse_error(p.origin))
                }
            }
            CommandThunk::Completed(_, c) => {
                self.push_command(c);
                Ok(())
            }
            CommandThunk::Start(c) => {
                self.partials.push(c);
                Ok(())
            }
            CommandThunk::End(p) => match (p.as_rule(), self.partials.pop()) {
                (
                    Rule::command_retn,
                    Some(Control::Subr {
                        label, definition, ..
                    }),
                ) => {
                    let c = Command::Subr { label, definition };
                    self.push_command(c);
                    Ok(())
                }
                (
                    Rule::command_repe,
                    Some(Control::Reps {
                        count, definition, ..
                    }),
                ) => {
                    let c = Command::Reps { count, definition };
                    self.push_command(c);
                    Ok(())
                }
                (Rule::command_ever, Some(Control::Loop { definition, .. })) => {
                    let c = Command::LoopEver { definition };
                    self.push_command(c);
                    Ok(())
                }
                (
                    Rule::command_next,
                    Some(Control::Econ {
                        agent, definition, ..
                    }),
                ) => {
                    let c = Command::Econ { agent, definition };
                    self.push_command(c);
                    Ok(())
                }
                (Rule::command_next, Some(Control::Enum(e))) => {
                    self.push_command(Command::Enum(e));
                    Ok(())
                }
                (Rule::command_next, Some(Control::Etch(e))) => {
                    self.push_command(Command::Etch(e));
                    Ok(())
                }

                (Rule::command_next, Some(Control::Esee(e))) => {
                    self.push_command(Command::Esee(e));
                    Ok(())
                }
                (Rule::command_next, Some(Control::Epas(e))) => {
                    self.push_command(Command::Epas(e));
                    Ok(())
                }
                (Rule::command_endi, Some(Control::DoIf(do_if))) => {
                    self.push_command(Command::Doif(do_if));
                    Ok(())
                }
                _ => Err(CaosError::new_parse_error(p)),
            },
            CommandThunk::EndLoop(p, condition) => {
                if let Some(Control::Loop { definition }) = self.partials.pop() {
                    self.push_command(Command::LoopUntl {
                        definition,
                        condition,
                    });
                    Ok(())
                } else {
                    Err(CaosError::new_parse_error(p))
                }
            }
            CommandThunk::StartElif(p, condition) => {
                if let Some(Control::DoIf(do_if)) = self.partials.last_mut() {
                    if do_if.else_definition.is_some() {
                        Err(CaosError::new_parse_error(p))
                    } else {
                        do_if.push_elif(condition, ScriptDefinition::default());
                        Ok(())
                    }
                } else {
                    Err(CaosError::new_parse_error(p))
                }
            }
            CommandThunk::StartElse(p) => {
                if let Some(Control::DoIf(do_if)) = self.partials.last_mut() {
                    if do_if.else_definition.is_some() {
                        Err(CaosError::new_parse_error(p))
                    } else {
                        do_if.set_else(ScriptDefinition::default());
                        Ok(())
                    }
                } else {
                    Err(CaosError::new_parse_error(p))
                }
            }
        }
    }
}
