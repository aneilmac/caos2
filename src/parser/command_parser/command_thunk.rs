use super::Control;
use crate::ast::{Command, Condition};
use crate::parser::Partial;
use crate::{CaosError, Rule};
use pest::iterators::Pair;

/// A thunk for a partial-parse of some expression.
/// May either be in the [CommandThunk::Completed] state where the thunk can be immediately
/// unwrapped into an [Command], or the [CommandThunk::Partial] state, where the expression
/// requires further arguments to complete.
pub(crate) enum CommandThunk<'i> {
    Completed(Pair<'i, Rule>, Command),
    Partial(Partial<'i, Command>),
    Start(Control),
    StartElif(Pair<'i, Rule>, Condition),
    StartElse(Pair<'i, Rule>),
    End(Pair<'i, Rule>),
    EndLoop(Pair<'i, Rule>, Condition),
}

impl<'i> CommandThunk<'i> {
    /// Returns `true` if the thunk can be completed/unwrapped.
    pub fn needs_expression(&self) -> bool {
        match self {
            Self::Partial(p) => !p.is_ready(),
            _ => false,
        }
    }
}
