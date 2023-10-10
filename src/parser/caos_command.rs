mod doif;
mod r#enum;
mod r#loop;
mod subr;

use crate::{ast::Command, CaosError, ErrorType, Rule};
use pest::iterators::Pair;
use doif::*;
use r#enum::*;
use r#loop::*;
use subr::*;

pub fn parse_command(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::command_doif => parse_command_doif(pair),
        Rule::command_subr => parse_command_subr(pair),
        Rule::command_reps => parse_command_reps(pair),
        Rule::command_econ => parse_command_econ(pair),
        Rule::command_enum => parse_command_enum(pair),
        Rule::command_etch => parse_command_etch(pair),
        Rule::command_esee => parse_command_esee(pair),
        Rule::command_epass => parse_command_epass(pair),
        Rule::command_loop_untl => parse_command_loop_untl(pair),
        Rule::command_loop_ever => parse_command_loop_ever(pair),
        _ => todo!()
    }
}
