use crate::{ast::CosFile, CaosError, ErrorType, Rule};
use pest::iterators::Pair;

pub fn parse_program(pair: Pair<Rule>) -> Result<CosFile, CaosError> {
    todo!()
}
