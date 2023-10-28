use super::CommandThunk;
use crate::Rule;
use pest::iterators::Pair;

/// Parses a given CAOS parser rule into a thunk which can be evaluated to a CAOS type.
pub(crate) trait CommandParser {
    fn parse_thunk<'i>(pair: Pair<'i, Rule>) -> Option<CommandThunk<'i>>;
}
