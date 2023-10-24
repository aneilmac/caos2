use crate::{
    ast::{Command, Condition, ScriptDefinition},
    parser::{condition::parse_condition, script::parse_script_contents},
    CaosError, Rule,
};
use pest::iterators::Pair;

pub fn parse_command_doif(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command_doif {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let condition: Condition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_condition)?;

    let definition: ScriptDefinition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    let elif_definitions = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_elif_parts)?;

    let else_definition = match it.next() {
        Some(pair) => Some(parse_else_part(pair)?),
        None => None,
    };

    Ok(Command::Doif {
        condition,
        definition,
        elif_definitions,
        else_definition,
    })
}

fn parse_else_part(pair: Pair<Rule>) -> Result<ScriptDefinition, CaosError> {
    if pair.as_rule() != Rule::command_else {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))?;
    parse_script_contents(pair)
}

fn parse_elif_parts(pair: Pair<Rule>) -> Result<Vec<(Condition, ScriptDefinition)>, CaosError> {
    todo!()
    // if pair.as_rule() != Rule::command_elif_parts {
    //     return Err(CaosError::new_parse_error(pair));
    // }
    // pair.into_inner().map(parse_elif).collect()
}

fn parse_elif(pair: Pair<Rule>) -> Result<(Condition, ScriptDefinition), CaosError> {
    if pair.as_rule() != Rule::command_elif {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let condition: Condition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_condition)?;

    let contents: ScriptDefinition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    Ok((condition, contents))
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::ConditionType,
        parser::{caos_command::parse_command, CaosParser},
    };
    use pest::Parser;

    use super::*;

    #[test]
    fn test_command_doif_parse_error_endi() {
        CaosParser::parse(Rule::command, "DOIF 1 <> 2").expect_err("No ENDI");
    }

    #[test]
    fn test_command_doif_empty() {
        for p in CaosParser::parse(Rule::command, "DOIF 1 <> 2 ENDI").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Doif {
                    condition: Condition::Simple {
                        cond_type: crate::ast::ConditionType::Ne,
                        lhs: 1.into(),
                        rhs: 2.into()
                    },
                    definition: ScriptDefinition { commands: vec![] },
                    elif_definitions: vec![],
                    else_definition: None
                },
            );
        }
    }

    #[test]
    fn test_command_doif_simple() {
        for p in CaosParser::parse(Rule::command, "DOIF 1 <> 2 NOHH OVER ENDI").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Doif {
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
                },
            );
        }
    }

    #[test]
    fn test_command_doif_empty_else() {
        for p in CaosParser::parse(Rule::command, "DOIF 1 <> 2 NOHH ELSE ENDI").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Doif {
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
                },
            );
        }
    }

    #[test]
    fn test_command_doif_else() {
        for p in
            CaosParser::parse(Rule::command, "DOIF 1 <> 2 NOHH ELSE OVER ENDI").expect("Parsed")
        {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Doif {
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
                },
            );
        }
    }

    #[test]
    fn test_command_doif_empty_elif() {
        for p in
            CaosParser::parse(Rule::command, "DOIF 1 <> 2 NOHH ELIF 3 < 4 ENDI").expect("Parsed")
        {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Doif {
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
                },
            );
        }
    }

    #[test]
    fn test_command_doif_elif() {
        for p in CaosParser::parse(Rule::command, "DOIF 1 <> 2 NOHH ELIF 3 < 4 BRN: DMPB ENDI")
            .expect("Parsed")
        {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Doif {
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
                },
            );
        }
    }

    #[test]
    fn test_command_doif_elif_multiple() {
        for p in CaosParser::parse(
            Rule::command,
            "DOIF 1 <> 2 NOHH ELIF 3 < 4 BRN: DMPB ELIF 5 = 6 OVER ENDI",
        )
        .expect("Parsed")
        {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Doif {
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
                },
            );
        }
    }

    #[test]
    fn test_command_doif_elif_else() {
        for p in CaosParser::parse(
            Rule::command,
            "DOIF 1 <> 2 NOHH ELIF 3 < 4 BRN: DMPB ELSE OVER ENDI",
        )
        .expect("Parsed")
        {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Doif {
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
                },
            );
        }
    }
}
