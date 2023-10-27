mod base;

//mod script;
mod command_parser;
mod condition_parser;
mod expression_parser;
mod partial;

use base::*;
// use caos_program::*;
// use script::*;
pub(crate) use command_parser::*;
pub(self) use condition_parser::*;
pub(crate) use expression_parser::*;
pub(crate) use partial::*;

use crate::{ast::CosFile, CaosError, ErrorType};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/base.pest"]
#[grammar = "grammar/caos_agent.pest"]
#[grammar = "grammar/caos_command.pest"]
#[grammar = "grammar/caos_float.pest"]
#[grammar = "grammar/caos_int.pest"]
#[grammar = "grammar/caos_program.pest"]
#[grammar = "grammar/caos_string.pest"]
#[grammar = "grammar/caos_variable.pest"]
#[grammar = "grammar/condition.pest"]
#[grammar = "grammar/caos_overloaded.pest"]
#[grammar = "grammar/script.pest"]
struct CaosParser;

pub fn parse_cos(cos_content: &str) -> Result<CosFile, CaosError> {
    todo!()
    // let res = CaosParser::parse(Rule::program, cos_content)
    //     .map_err(|e| CaosError::new_from_error(Box::new(e)))?
    //     .next()
    //     .ok_or(CaosError::new(
    //         ErrorType::ParseError { line_col: (0, 0) },
    //         String::from("Unknown parsing error"),
    //     ))?;
    // parse_program(res)
}
