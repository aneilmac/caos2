use super::Partial;
use crate::{
    ast::{Anything, Command},
    CaosError,
};

/// A thunk for a partial-parse of some expression.
/// May either be in the [CommandThunk::Completed] state where the thunk can be immediately
/// unwrapped into an [Command], or the [CommandThunk::Partial] state, where the expression
/// requires further arguments to complete.
pub(crate) enum CommandThunk<'i> {
    CompletedCommand(Command),
    PartialCommand(Partial<'i, Command>),
}

impl<'i> CommandThunk<'i> {
    /// Returns `true` if the thunk can be completed/unwrapped.
    pub fn is_ready(&self) -> bool {
        match self {
            Self::CompletedCommand(_) => true,
            Self::PartialCommand(p) => p.is_ready(),
        }
    }

    /// Attempts to complete/unwrap the thunk, producing an [Anything].
    pub fn complete(self) -> Result<Command, CaosError> {
        match self {
            Self::CompletedCommand(c) => Ok(c),
            Self::PartialCommand(p) => p.complete(),
        }
    }
}

impl From<Command> for CommandThunk<'_> {
    fn from(value: Command) -> Self {
        CommandThunk::CompletedCommand(value)
    }
}
