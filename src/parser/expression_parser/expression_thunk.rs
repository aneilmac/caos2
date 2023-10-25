use crate::{ast::Anything, CaosError, Rule};
use pest::iterators::Pair;
use std::vec::Vec;

/// A thunk for a partial-parse of some expression.
/// May either be in the [ExpressionThunk::Completed] state where the thunk can be immediately
/// unwrapped into an [Anything], or the [ExpressionThunk::Partial] state, where the expression
/// requires further arguments to complete.
pub(crate) enum ExpressionThunk<'i> {
    Completed(Anything),
    Partial(Partial<'i>),
}

impl<'i> ExpressionThunk<'i> {
    /// Returns `true` if the thunk can be completed/unwrapped.
    pub fn is_ready(&self) -> bool {
        match self {
            Self::Completed(_) => true,
            Self::Partial(p) => p.is_ready(),
        }
    }

    /// Attempts to complete/unwrap the thunk, producing an [Anything].
    pub fn complete(self) -> Result<Anything, CaosError> {
        match self {
            Self::Completed(a) => Ok(a),
            Self::Partial(p) => p.complete(),
        }
    }
}

impl From<Anything> for ExpressionThunk<'_> {
    fn from(value: Anything) -> Self {
        ExpressionThunk::Completed(value)
    }
}

/// A [Partial] is contents of an [ExpressionThunk] which is not yet ready to be completed.
/// This object requires a number of arguments to be pushed into [Partial::arg_parts] matching the
/// count of [Partial::target_args].
pub(crate) struct Partial<'i> {
    pub origin: Pair<'i, Rule>,
    pub arg_parts: Vec<Anything>,
    pub target_args: usize,
    pub complete_method: Box<dyn Fn(Pair<'i, Rule>, Vec<Anything>) -> Result<Anything, CaosError>>,
}

impl<'i> Partial<'i> {
    /// Returns true if the partial has enough arguments to complete/unwrap.
    pub fn is_ready(&self) -> bool {
        self.arg_parts.len() == self.target_args
    }

    /// Completes/unwraps the partial.
    pub fn complete(self) -> Result<Anything, CaosError> {
        (self.complete_method)(self.origin, self.arg_parts)
    }
}
