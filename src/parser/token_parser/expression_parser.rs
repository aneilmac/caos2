use super::ExpressionThunk;
use crate::Rule;
use pest::iterators::Pair;

/// Parses a given CAOS parser rule into a thunk which can be evaluated to a CAOS type.
pub(crate) trait ExpressionParser {
    fn parse_thunk<'i>(pair: Pair<'i, Rule>) -> Option<ExpressionThunk<'i>>;
}
