use super::*;
use crate::ast::{Condition, ConditionType, ScriptDefinition};
use pest::Parser;

#[test]
fn test_command_reps_parse_error_repe() {
    let mut p = CaosParser::parse(Rule::command, "REPS 0 BRN: DMPB").expect("Successful parse");
    parse_commands(&mut p).expect_err("No REPE");
}

#[test]
fn test_command_loop_error_parse() {
    let mut p = CaosParser::parse(Rule::command, "LOOP BRN: DMPB").expect("Successful parse");
    parse_commands(&mut p).expect_err("No UNTL or Ever");
}

#[test]
fn test_command_loop_untl_error_parse() {
    let mut p = CaosParser::parse(Rule::command, "LOOP BRN: DMPB UNTL").expect("Successful parse");
    parse_commands(&mut p).expect_err("No Condition");
}

#[test]
fn test_command_reps_empty() {
    assert_eq!(
        parse_cmnd("REPS 0 REPE"),
        Command::Reps {
            count: Box::new(0.into()),
            definition: ScriptDefinition::default()
        },
    );
}

#[test]
fn test_command_reps() {
    assert_eq!(
        parse_cmnd("REPS 0 BRN: DMPB REPE"),
        Command::Reps {
            count: Box::new(0.into()),
            definition: ScriptDefinition {
                commands: vec![Command::BrnDmpb]
            }
        },
    );
}

#[test]
fn test_command_loop_ever_empty() {
    assert_eq!(
        parse_cmnd("LOOP EVER"),
        Command::LoopEver {
            definition: ScriptDefinition { commands: vec![] }
        },
    );
}

#[test]
fn test_command_loop_ever() {
    assert_eq!(
        parse_cmnd("LOOP BRN: DMPB EVER"),
        Command::LoopEver {
            definition: ScriptDefinition {
                commands: vec![Command::BrnDmpb]
            }
        },
    );
}

#[test]
fn test_command_loop_untl_empty() {
    assert_eq!(
        parse_cmnd("LOOP UNTL 1 <> 2"),
        Command::LoopUntl {
            definition: ScriptDefinition { commands: vec![] },
            condition: Condition::Simple {
                cond_type: ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            }
        },
    );
}

#[test]
fn test_command_loop_untl() {
    assert_eq!(
        parse_cmnd("LOOP BRN: DMPB UNTL 1 <> 2"),
        Command::LoopUntl {
            definition: ScriptDefinition {
                commands: vec![Command::BrnDmpb]
            },
            condition: Condition::Simple {
                cond_type: ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            }
        },
    );
}
