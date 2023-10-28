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
        ast::{Command, Label, ScriptDefinition},
        parser::CaosParser,
    };
    use pest::Parser;

    fn parse_program_str(content: &str) -> CosFile {
        let mut ps = CaosParser::parse(Rule::program, content).expect("Parsed");
        parse_program(ps.next().unwrap()).expect("Successful parse")
    }

    #[test]
    fn test_program_empty() {
        assert_eq!(parse_program_str(""), CosFile::default());
    }

    #[test]
    fn test_program_implicit_install_no_end_tag() {
        assert_eq!(
            parse_program_str("BRN: DMPB"),
            CosFile {
                scripts: vec![Script::Install(ScriptDefinition {
                    commands: vec![Command::BrnDmpb]
                })]
            }
        );
    }

    #[test]
    fn test_program_implicit_install_with_end_tag() {
        assert_eq!(
            parse_program_str("BRN: DMPB ENDM"),
            CosFile {
                scripts: vec![Script::Install(ScriptDefinition {
                    commands: vec![Command::BrnDmpb]
                })]
            }
        );
    }

    #[test]
    fn test_program_multi_scripts() {
        assert_eq!(
            parse_program_str("ISCR BRN: DMPB ENDM RSCR OVER ENDM"),
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

    #[test]
    fn test_program_multi_scripts_no_end_tags() {
        assert_eq!(
            parse_program_str("ISCR BRN: DMPB RSCR OVER"),
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

    #[test]
    fn test_program_implit_install_explicit_removal() {
        assert_eq!(
            parse_program_str("BRN: DMPB RSCR OVER ENDM"),
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

    #[test]
    fn test_keyword_label() {
        assert_eq!(
            parse_program_str("ISCR SUBR ENDM RETN ENDM"),
            CosFile {
                scripts: vec![Script::Install(ScriptDefinition {
                    commands: vec![Command::Subr {
                        label: String::from("ENDM").into(),
                        definition: ScriptDefinition::default()
                    }]
                })]
            }
        );
    }

    #[test]
    fn test_comment_no_newline() {
        assert_eq!(
            parse_program_str(r#"* Comment without newline"#),
            CosFile::default());
    }
}
