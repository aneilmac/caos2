mod doif_tests;
mod enum_tests;
mod loop_tests;
mod simple_command_tests;
mod subr_tests;

use pest::Parser;

use crate::{ast::Command, parser::parse_commands, parser::CaosParser, Rule};

fn parse_cmnd(content: &str) -> Command {
    let mut p = CaosParser::parse(Rule::tokens, content).expect("Successful pest parse");
    let p = parse_commands(&mut p).expect("Successful command parse");
    assert_eq!(1, p.len(), "Expected only 1 parsed command");
    p.into_iter().next().unwrap()
}
