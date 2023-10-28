use super::*;
use crate::ast::ScriptDefinition;
use pest::Parser;

#[test]
fn test_command_subr_empty_parse_error_label() {
    let mut p = CaosParser::parse(Rule::command, "SUBR RETN").expect("Parsed");
    parse_commands(&mut p).expect_err("No label");
}

#[test]
fn test_command_subr_empty_parse_error_retn() {
    let mut p = CaosParser::parse(Rule::command, "SUBR FOO").expect("Parsed");
    parse_commands(&mut p).expect_err("No RETN");
}

#[test]
fn test_command_subr_empty() {
    assert_eq!(
        parse_cmnd("SUBR FOO RETN"),
        Command::Subr {
            label: String::from("FOO").into(),
            definition: ScriptDefinition::default()
        },
    );
}

#[test]
fn test_command_subr() {
    assert_eq!(
        parse_cmnd("SUBR FOO BRN: DMPB RETN"),
        Command::Subr {
            label: String::from("FOO").into(),
            definition: ScriptDefinition {
                commands: vec![Command::BrnDmpb]
            }
        },
    );
}
