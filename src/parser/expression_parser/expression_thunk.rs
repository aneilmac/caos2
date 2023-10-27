use super::Partial;
use crate::{ast::Anything, CaosError, Rule};
use pest::iterators::Pair;

/// A thunk for a partial-parse of some expression.
/// May either be in the [ExpressionThunk::Completed] state where the thunk can be immediately
/// unwrapped into an [Anything], or the [ExpressionThunk::Partial] state, where the expression
/// requires further arguments to complete.
pub(crate) enum ExpressionThunk<'i> {
    Completed(Pair<'i, Rule>, Anything),
    Partial(Partial<'i, Anything>),
}

impl<'i> ExpressionThunk<'i> {
    /// Returns `true` if the thunk can be completed/unwrapped.
    pub fn is_ready(&self) -> bool {
        match self {
            Self::Completed(..) => true,
            Self::Partial(p) => p.is_ready(),
        }
    }

    /// Attempts to complete/unwrap the thunk, producing an [Anything].
    pub fn complete(self) -> Result<Anything, CaosError> {
        match self {
            Self::Completed(_, a) => Ok(a),
            Self::Partial(p) => p.complete(),
        }
    }

    pub fn to_pair(self) -> Pair<'i, Rule> {
        match self {
            Self::Completed(p, _) => p,
            Self::Partial(p) => p.origin,
        }
    }
}
