use crate::{
    ast::{CosFile, Script},
    CaosError, Rule,
};
use pest::iterators::Pair;

use super::{parse_script_contents, script::parse_script};

pub fn parse_program(pair: Pair<Rule>) -> Result<CosFile, CaosError> {
    if pair.as_rule() != Rule::program {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let implicit_script = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_implict_install_script)?;

    let scripts = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(|pair| parse_scripts(pair, implicit_script))?;

    Ok(CosFile { scripts })
}

fn parse_implict_install_script(pair: Pair<Rule>) -> Result<Option<Script>, CaosError> {
    if pair.as_rule() != Rule::implicit_install_script {
        return Err(CaosError::new_parse_error(pair));
    }

    let contents = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    if contents.commands.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Script::Install(contents)))
    }
}

fn parse_scripts(pair: Pair<Rule>, implicit: Option<Script>) -> Result<Vec<Script>, CaosError> {
    if pair.as_rule() != Rule::scripts {
        return Err(CaosError::new_parse_error(pair));
    }

    let it = pair.into_inner().map(parse_script);
    match implicit {
        Some(s) => std::iter::once(Ok(s)).chain(it).collect(),
        None => it.collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{Command, ScriptDefinition},
        parser::CaosParser,
    };
    use pest::Parser;

    #[test]
    fn test_program_empty() {
        for p in CaosParser::parse(Rule::program, "").expect("Parsed") {
            assert_eq!(
                parse_program(p).expect("Parsed program file"),
                CosFile::default()
            );
        }
    }

    #[test]
    fn test_program_implicit_install_no_end_tag() {
        for p in CaosParser::parse(Rule::program, "BRN: DMPB").expect("Parsed") {
            assert_eq!(
                parse_program(p).expect("Parsed program file"),
                CosFile {
                    scripts: vec![Script::Install(ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    })]
                }
            );
        }
    }

    #[test]
    fn test_program_implicit_install_with_end_tag() {
        for p in CaosParser::parse(Rule::program, "BRN: DMPB ENDM").expect("Parsed") {
            assert_eq!(
                parse_program(p).expect("Parsed program file"),
                CosFile {
                    scripts: vec![Script::Install(ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    })]
                }
            );
        }
    }

    #[test]
    fn test_program_multi_scripts() {
        for p in
            CaosParser::parse(Rule::program, "ISCR BRN: DMPB ENDM RSCR OVER ENDM").expect("Parsed")
        {
            assert_eq!(
                parse_program(p).expect("Parsed program file"),
                CosFile {
                    scripts: vec![
                        Script::Install(ScriptDefinition {
                            commands: vec![Command::BrnDmpb]
                        }),
                        Script::Removal(ScriptDefinition {
                            commands: vec![Command::Over]
                        })
                    ]
                }
            );
        }
    }

    #[test]
    fn test_program_multi_scripts_no_end_tags() {
        for p in CaosParser::parse(Rule::program, "ISCR BRN: DMPB RSCR OVER").expect("Parsed") {
            assert_eq!(
                parse_program(p).expect("Parsed program file"),
                CosFile {
                    scripts: vec![
                        Script::Install(ScriptDefinition {
                            commands: vec![Command::BrnDmpb]
                        }),
                        Script::Removal(ScriptDefinition {
                            commands: vec![Command::Over]
                        })
                    ]
                }
            );
        }
    }

    #[test]
    fn test_program_implit_install_explicit_removal() {
        for p in CaosParser::parse(Rule::program, "BRN: DMPB RSCR OVER ENDM").expect("Parsed") {
            assert_eq!(
                parse_program(p).expect("Parsed program file"),
                CosFile {
                    scripts: vec![
                        Script::Install(ScriptDefinition {
                            commands: vec![Command::BrnDmpb]
                        }),
                        Script::Removal(ScriptDefinition {
                            commands: vec![Command::Over]
                        })
                    ]
                }
            );
        }
    }
}
