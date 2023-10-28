use super::*;
use crate::ast::{Condition, ConditionType, DoIf, ScriptDefinition};
use pest::Parser;

#[test]
fn test_command_doif_parse_error_endi() {
    let mut p = CaosParser::parse(Rule::command, "DOIF 1 <> 2").expect("Successful parse");
    parse_commands(&mut p).expect_err("No ENDI");
}

#[test]
fn test_command_doif_elif_else_error_order() {
    let mut p = CaosParser::parse(Rule::command, "DOIF 1 <> 2 NOHH ELSE OVER ELIF 3 < 4 BRN: DMPB END").expect("Successful parse");
    parse_commands(&mut p).expect_err("ELSE and ELIF in wrong order");
}

#[test]
fn test_command_doif_empty() {
    assert_eq!(
        parse_cmnd("DOIF 1 <> 2 ENDI"),
        Command::Doif(DoIf {
            condition: Condition::Simple {
                cond_type: ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            },
            definition: ScriptDefinition::default(),
            elif_definitions: vec![],
            else_definition: None
        },)
    );
}

#[test]
fn test_command_doif_simple() {
    assert_eq!(
        parse_cmnd("DOIF 1 <> 2 NOHH OVER ENDI"),
        Command::Doif(DoIf {
            condition: Condition::Simple {
                cond_type: crate::ast::ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            },
            definition: ScriptDefinition {
                commands: vec![Command::Nohh, Command::Over]
            },
            elif_definitions: vec![],
            else_definition: None
        },)
    );
}

#[test]
fn test_command_doif_empty_else() {
    assert_eq!(
        parse_cmnd("DOIF 1 <> 2 NOHH ELSE ENDI"),
        Command::Doif(DoIf {
            condition: Condition::Simple {
                cond_type: crate::ast::ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            },
            definition: ScriptDefinition {
                commands: vec![Command::Nohh]
            },
            elif_definitions: vec![],
            else_definition: Some(ScriptDefinition { commands: vec![] })
        },)
    );
}

#[test]
fn test_command_doif_else() {
    assert_eq!(
        parse_cmnd("DOIF 1 <> 2 NOHH ELSE OVER ENDI"),
        Command::Doif(DoIf {
            condition: Condition::Simple {
                cond_type: crate::ast::ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            },
            definition: ScriptDefinition {
                commands: vec![Command::Nohh]
            },
            elif_definitions: vec![],
            else_definition: Some(ScriptDefinition {
                commands: vec![Command::Over]
            })
        },)
    );
}

#[test]
fn test_command_doif_empty_elif() {
    assert_eq!(
        parse_cmnd("DOIF 1 <> 2 NOHH ELIF 3 < 4 ENDI"),
        Command::Doif(DoIf {
            condition: Condition::Simple {
                cond_type: ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            },
            definition: ScriptDefinition {
                commands: vec![Command::Nohh]
            },
            elif_definitions: vec![(
                Condition::Simple {
                    cond_type: ConditionType::Lt,
                    lhs: 3.into(),
                    rhs: 4.into()
                },
                ScriptDefinition { commands: vec![] }
            )],
            else_definition: None
        },)
    );
}

#[test]
fn test_command_doif_elif() {
    assert_eq!(
        parse_cmnd("DOIF 1 <> 2 NOHH ELIF 3 < 4 BRN: DMPB ENDI"),
        Command::Doif(DoIf {
            condition: Condition::Simple {
                cond_type: ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            },
            definition: ScriptDefinition {
                commands: vec![Command::Nohh]
            },
            elif_definitions: vec![(
                Condition::Simple {
                    cond_type: ConditionType::Lt,
                    lhs: 3.into(),
                    rhs: 4.into()
                },
                ScriptDefinition {
                    commands: vec![Command::BrnDmpb]
                }
            )],
            else_definition: None
        },)
    );
}

#[test]
fn test_command_doif_elif_multiple() {
    assert_eq!(
        parse_cmnd("DOIF 1 <> 2 NOHH ELIF 3 < 4 BRN: DMPB ELIF 5 = 6 OVER ENDI"),
        Command::Doif(DoIf {
            condition: Condition::Simple {
                cond_type: ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            },
            definition: ScriptDefinition {
                commands: vec![Command::Nohh]
            },
            elif_definitions: vec![
                (
                    Condition::Simple {
                        cond_type: ConditionType::Lt,
                        lhs: 3.into(),
                        rhs: 4.into()
                    },
                    ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    }
                ),
                (
                    Condition::Simple {
                        cond_type: ConditionType::Eq,
                        lhs: 5.into(),
                        rhs: 6.into()
                    },
                    ScriptDefinition {
                        commands: vec![Command::Over]
                    }
                )
            ],
            else_definition: None
        }),
    );
}

#[test]
fn test_command_doif_elif_else() {
    assert_eq!(
        parse_cmnd("DOIF 1 <> 2 NOHH ELIF 3 < 4 BRN: DMPB ELSE OVER ENDI"),
        Command::Doif(DoIf {
            condition: Condition::Simple {
                cond_type: ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            },
            definition: ScriptDefinition {
                commands: vec![Command::Nohh]
            },
            elif_definitions: vec![(
                Condition::Simple {
                    cond_type: ConditionType::Lt,
                    lhs: 3.into(),
                    rhs: 4.into()
                },
                ScriptDefinition {
                    commands: vec![Command::BrnDmpb]
                }
            )],
            else_definition: Some(ScriptDefinition {
                commands: vec![Command::Over]
            })
        },)
    );
}
