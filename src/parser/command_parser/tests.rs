use pest::Parser;

use crate::{ast::Command, parser::parse_commands, parser::CaosParser, Rule};

mod simple_command_tests;

fn parse_cmnd(content: &str) -> Command {
    let mut p = CaosParser::parse(Rule::tokens, content).expect("Successful pest parse");
    let p = parse_commands(&mut p).expect("Successful command parse");
    assert_eq!(1, p.len(), "Expected only 1 parsed command");
    p.into_iter().next().unwrap()
}
