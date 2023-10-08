use crate::{ast::Condition, CaosError, ErrorType, Rule};
use pest::iterators::Pair;

pub fn parse_condition(pair: Pair<Rule>) -> Result<Condition, CaosError> {
    todo!()
}
