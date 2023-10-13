use crate::{
    ast::Command,
    parser::{base::parse_label, script::parse_script_contents},
    CaosError, Rule,
};
use pest::iterators::Pair;

pub fn parse_command_subr(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command_subr {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let label = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_label)?;

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    Ok(Command::Subr { label, definition })
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::ScriptDefinition,
        parser::{caos_command::parse_command, CaosParser},
    };
    use pest::Parser;

    use super::*;

    #[test]
    fn test_command_subr_empty_parse_error_label() {
        CaosParser::parse(Rule::command, "SUBR RETN").expect_err("No label");
    }

    #[test]
    fn test_command_subr_empty_parse_error_retn() {
        CaosParser::parse(Rule::command, "SUBR FOO").expect_err("No RETN");
    }

    #[test]
    fn test_command_subr_empty() {
        for p in CaosParser::parse(Rule::command, "SUBR FOO RETN").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Subr {
                    label: String::from("FOO").into(),
                    definition: ScriptDefinition::default()
                },
            );
        }
    }

    #[test]
    fn test_command_subr() {
        for p in CaosParser::parse(Rule::command, "SUBR FOO BRN: DMPB RETN").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Subr {
                    label: String::from("FOO").into(),
                    definition: ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    }
                },
            );
        }
    }
}
