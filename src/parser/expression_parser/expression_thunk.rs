use super::Partial;
use crate::{ast::Anything, Rule};
use pest::iterators::Pair;

/// A thunk for a partial-parse of some expression.
/// May either be in the [ExpressionThunk::Completed] state where the thunk can be immediately
/// unwrapped into an [Anything], or the [ExpressionThunk::Partial] state, where the expression
/// requires further arguments to complete.
pub(crate) enum ExpressionThunk<'i> {
    Completed(Pair<'i, Rule>, Anything),
    Partial(Partial<'i, Anything>),
}

