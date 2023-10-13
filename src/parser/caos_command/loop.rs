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
    if pair.as_rule() != Rule::command_loop_ever {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    Ok(Command::LoopEver { definition })
}

pub fn parse_command_loop_untl(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command_loop_untl {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    let condition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_condition)?;

    Ok(Command::LoopUntl {
        definition,
        condition,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{ConditionType, ScriptDefinition},
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
}
