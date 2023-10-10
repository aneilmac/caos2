use crate::{ast::Command, CaosError, Rule};
use pest::iterators::Pair;

pub fn parse_command_doif(pair: Pair<Rule>) -> Result<Command, CaosError> {
    todo!()
}
