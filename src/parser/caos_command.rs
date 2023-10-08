use crate::{ast::Command, CaosError, ErrorType, Rule};
use pest::iterators::Pair;

pub fn parse_command(pair: Pair<Rule>) -> Result<Command, CaosError> {
    todo!()
}
