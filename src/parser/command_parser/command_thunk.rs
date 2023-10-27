use crate::parser::Partial;
use crate::{ast::Command, CaosError, Rule};
use pest::iterators::Pair;

/// A thunk for a partial-parse of some expression.
/// May either be in the [CommandThunk::Completed] state where the thunk can be immediately
/// unwrapped into an [Command], or the [CommandThunk::Partial] state, where the expression
/// requires further arguments to complete.
pub(crate) enum CommandThunk<'i> {
    Completed(Pair<'i, Rule>, Command),
    Partial(Partial<'i, Command>),
}

impl<'i> CommandThunk<'i> {
    /// Returns `true` if the thunk can be completed/unwrapped.
    pub fn is_ready(&self) -> bool {
        match self {
            Self::Completed(..) => true,
            Self::Partial(p) => p.is_ready(),
        }
    }

    /// Attempts to complete/unwrap the thunk, producing an [Anything].
    pub fn complete(self) -> Result<Command, CaosError> {
        match self {
            Self::Completed(_, c) => Ok(c),
            Self::Partial(p) => p.complete(),
        }
    }
}
