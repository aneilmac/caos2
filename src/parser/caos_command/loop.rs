use crate::{ast::Command, CaosError, Rule};
use pest::iterators::Pair;

pub fn parse_command_loop_untl(pair: Pair<Rule>) -> Result<Command, CaosError> {
    todo!()
}

pub fn parse_command_loop_ever(pair: Pair<Rule>) -> Result<Command, CaosError> {
    todo!()
}

pub fn parse_command_reps(pair: Pair<Rule>) -> Result<Command, CaosError> {
    todo!()
}