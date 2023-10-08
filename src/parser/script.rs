use crate::{ast::Script, CaosError, ErrorType, Rule};
use pest::iterators::Pair;

pub fn parse_script(pair: Pair<Rule>) -> Result<Script, CaosError> {
    todo!()
}
