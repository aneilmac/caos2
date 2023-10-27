mod base;
//mod caos_agent;
//mod caos_command;
//mod caos_decimal;
//mod caos_float;
//mod caos_int;
//mod caos_program;
//mod caos_string;
//mod caos_variable;
//mod script;
mod condition_parser;
mod expression_parser;

use base::*;
// use caos_command::*;
// use caos_program::*;
// use script::*;
use condition_parser::*;
pub(crate) use expression_parser::*;

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
