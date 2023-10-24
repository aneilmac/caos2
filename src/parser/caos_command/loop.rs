use crate::{
    ast::Command,
    parser::{caos_int::parse_int_arg, condition::parse_condition, script::parse_script_contents},
    CaosError, Rule,
};
use pest::iterators::Pair;

pub fn parse_command_reps(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command_reps {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let count = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_int_arg)?;

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    Ok(Command::Reps { count, definition })
}

pub fn parse_command_loop_ever(pair: Pair<Rule>) -> Result<Command, CaosError> {
    todo!()
    // if pair.as_rule() != Rule::command_loop_ever {
    //     return Err(CaosError::new_parse_error(pair));
    // }

    // let mut it = pair.clone().into_inner();

    // let definition = it
    //     .next()
    //     .ok_or(CaosError::new_parse_error(pair.clone()))
    //     .and_then(parse_script_contents)?;

    // Ok(Command::LoopEver { definition })
}

pub fn parse_command_loop_untl(pair: Pair<Rule>) -> Result<Command, CaosError> {
    todo!()
    // if pair.as_rule() != Rule::command_loop_untl {
    //     return Err(CaosError::new_parse_error(pair));
    // }

    // let mut it = pair.clone().into_inner();

    // let definition = it
    //     .next()
    //     .ok_or(CaosError::new_parse_error(pair.clone()))
    //     .and_then(parse_script_contents)?;

    // let condition = it
    //     .next()
    //     .ok_or(CaosError::new_parse_error(pair.clone()))
    //     .and_then(parse_condition)?;

    // Ok(Command::LoopUntl {
    //     definition,
    //     condition,
    // })
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Condition, ConditionType, ScriptDefinition},
        parser::{caos_command::parse_command, CaosParser},
    };
    use pest::Parser;

    use super::*;

    #[test]
    fn test_command_reps_parse_error_repe() {
        CaosParser::parse(Rule::command, "REPS 0 BRN: DMPB").expect_err("No REPE");
    }

    #[test]
    fn test_command_reps_empty() {
        for p in CaosParser::parse(Rule::command, "REPS 0 REPE").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Reps {
                    count: 0.into(),
                    definition: ScriptDefinition::default()
                },
            );
        }
    }

    #[test]
    fn test_command_reps() {
        for p in CaosParser::parse(Rule::command, "REPS 0 BRN: DMPB REPE").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Reps {
                    count: 0.into(),
                    definition: ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    }
                },
            );
        }
    }

    #[test]
    fn test_command_loop_error_parse() {
        CaosParser::parse(Rule::command, "LOOP BRN: DMPB").expect_err("No UNTL or Ever");
    }

    #[test]
    fn test_command_loop_ever_empty() {
        for p in CaosParser::parse(Rule::command, "LOOP EVER").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::LoopEver {
                    definition: ScriptDefinition { commands: vec![] }
                },
            );
        }
    }

    #[test]
    fn test_command_loop_ever() {
        for p in CaosParser::parse(Rule::command, "LOOP BRN: DMPB EVER").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::LoopEver {
                    definition: ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    }
                },
            );
        }
    }

    #[test]
    fn test_command_loop_untl_error_parse() {
        CaosParser::parse(Rule::command, "LOOP BRN: DMPB UNTL").expect_err("No Condition");
    }

    #[test]
    fn test_command_loop_untl_empty() {
        for p in CaosParser::parse(Rule::command, "LOOP UNTL 1 <> 2").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
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
    }

    #[test]
    fn test_command_loop_untl() {
        for p in CaosParser::parse(Rule::command, "LOOP BRN: DMPB UNTL 1 <> 2").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
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
    }
}
