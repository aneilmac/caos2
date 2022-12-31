use nom::{
    bytes::complete::tag_no_case,
    combinator::{all_consuming, consumed, eof, opt},
    error::{FromExternalError, VerboseError},
    multi::separated_list0,
    sequence::tuple,
    Finish,
};

use super::{caos_skippable0, caos_skippable1};
use crate::{
    parser::{CaosParsable, CaosParseResult, EventScriptDefinition, Script, ScriptDefinition},
    CaosError, ErrorType,
};

#[derive(Debug, Eq, PartialEq)]
pub struct CosFile {
    pub install_script: Option<ScriptDefinition>,
    pub removal_script: Option<ScriptDefinition>,
    pub event_scripts: Vec<EventScriptDefinition>,
}

impl CosFile {
    pub fn parse(input: &str) -> Result<(&str, Self), VerboseError<&str>> {
        let do_parse = |input| {
            let (input, _) = caos_skippable0(input)?;
            let (input, me) = Self::parse_caos(input)?;
            let (input, _) = caos_skippable0(input)?;
            Ok((input, me))
        };
        all_consuming(do_parse)(input).finish()
    }
}

impl CaosParsable for CosFile {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self> {
        // A COS file may have an "implicit" install script if it contains commands before any
        // script tags.
        let (input, implicit_install_script) = opt(Script::parse_definition)(input)?;

        // Weird corner case of old CAOS parser:
        // Implicit installation scripts may still be closed with an `endm` tag.
        let (input, _) = opt(tuple((caos_skippable1, tag_no_case(Script::ENDM_TAG))))(input)?;

        let mut install_script: Option<ScriptDefinition> = None;
        if let Some(i) = implicit_install_script {
            if i.commands.len() > 0 {
                install_script = Some(i);
            }
        }

        let (input, s) = opt(tuple((caos_skippable0, eof)))(input)?;
        if let Some(_) = s {
            // COS file comprises entirely on an implicit install script. Early exit.
            return Ok((
                input,
                Self {
                    install_script: install_script,
                    removal_script: None,
                    event_scripts: vec![],
                },
            ));
        }

        // If implicit install script exists must be ended by a space before collection
        // into remainder.
        let input = if install_script.is_some() {
            let (input, _) = caos_skippable1(input)?;
            input
        } else {
            input
        };

        // Now we do the regular script collection.

        let (input, scripts) =
            separated_list0(caos_skippable1, consumed(Script::parse_caos))(input)?;

        let mut removal_script: Option<ScriptDefinition> = None;
        let mut event_scripts: Vec<EventScriptDefinition> = vec![];

        for (consumed, script) in scripts {
            match script {
                Script::Install(i) => {
                    if install_script.is_none() {
                        install_script = Some(i);
                    } else {
                        return Err(nom::Err::Failure(VerboseError::from_external_error(
                            consumed,
                            nom::error::ErrorKind::Complete,
                            CaosError::new(ErrorType::TooManyInstallScripts),
                        )));
                    }
                }
                Script::Removal(r) => {
                    if removal_script.is_none() {
                        removal_script = Some(r);
                    } else {
                        return Err(nom::Err::Failure(VerboseError::from_external_error(
                            consumed,
                            nom::error::ErrorKind::Complete,
                            CaosError::new(ErrorType::TooManyRemovalScripts),
                        )));
                    }
                }
                Script::Event(e) => {
                    event_scripts.push(e);
                }
            }
        }

        Ok((
            input,
            Self {
                install_script,
                removal_script,
                event_scripts,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::{
        Anything, Command, Condition, ConditionType, Decimal, FloatArg, IntArg, Integer, Variable,
    };

    use super::*;

    const CONTENT: &str = r#"
    * Simple test
    * Of multiline strings.
    inst
    **moisture monitor
    new: simp 1 1 114 "blnk" 1 0 0
    tick 9
    
    ******CREATE SOME MUCK - to seed the area
    reps 100
        new: simp 2 10 37 "graz" 2 218 3000
        attr 192
        elas 0
        setv va00 rand 0 2
        doif va00 eq 0
            accg 0.1
        elif va00 eq 1
            accg 0.3
        else
            accg 0.4
        endi
    
    
        mvto rand 217 2787 1840
        perm rand 0 70
    repe
    "#;

    #[test]
    fn test_empty() {
        let input = "";
        let (_, res) = CosFile::parse_caos(input).expect("Successful parse");
        assert_eq!(
            res,
            CosFile {
                install_script: None,
                removal_script: None,
                event_scripts: vec![]
            }
        );
    }

    #[test]
    fn test_only_install_script() {
        let input = "iscr\ninst\nendm";
        let (_, res) = CosFile::parse_caos(input).expect("Successful parse");
        assert_eq!(
            res,
            CosFile {
                install_script: Some(ScriptDefinition {
                    commands: vec![Command::Inst]
                }),
                removal_script: None,
                event_scripts: vec![]
            }
        );
    }

    #[test]
    fn test_implicit_install_into_removal() {
        let input = "inst\nrscr\ninst\nendm";
        let (_, res) = CosFile::parse_caos(input).expect("Successful parse");
        assert_eq!(
            res,
            CosFile {
                install_script: Some(ScriptDefinition {
                    commands: vec![Command::Inst]
                }),
                removal_script: Some(ScriptDefinition {
                    commands: vec![Command::Inst]
                }),
                event_scripts: vec![]
            }
        );
    }

    #[test]
    fn test_implicit_install_into_scrp() {
        let input = "inst\nscrp 4 0 0 34\ninst\nendm";
        let (s, res) = CosFile::parse_caos(input).expect("Successful parse");
        assert_eq!(s, "");
        assert_eq!(
            res,
            CosFile {
                install_script: Some(ScriptDefinition {
                    commands: vec![Command::Inst]
                }),
                removal_script: None,
                event_scripts: vec![EventScriptDefinition {
                    definition: ScriptDefinition {
                        commands: vec![Command::Inst]
                    },
                    family: 4,
                    genus: 0,
                    species: 0,
                    script_number: 34
                },]
            }
        );
    }

    #[test]
    fn test_install_into_scrp() {
        let input = "iscr\ninst\nscrp 4 0 0 34\ninst\nendm";
        let (s, res) = CosFile::parse_caos(input).expect("Successful parse");
        assert_eq!(s, "");
        assert_eq!(
            res,
            CosFile {
                install_script: Some(ScriptDefinition {
                    commands: vec![Command::Inst]
                }),
                removal_script: None,
                event_scripts: vec![EventScriptDefinition {
                    definition: ScriptDefinition {
                        commands: vec![Command::Inst]
                    },
                    family: 4,
                    genus: 0,
                    species: 0,
                    script_number: 34
                },]
            }
        );
    }

    #[test]
    fn test_only_removal_script() {
        let input = "rscr\ninst\nendm";
        let (_, res) = CosFile::parse_caos(input).expect("Successful parse");
        assert_eq!(
            res,
            CosFile {
                removal_script: Some(ScriptDefinition {
                    commands: vec![Command::Inst]
                }),
                install_script: None,
                event_scripts: vec![]
            }
        );
    }

    #[test]
    fn test_event_scripts() {
        let input = "scrp 4 0 0 34\ninst\nendm\nscrp 4 0 0 34\ninst\nendm";
        let (_, res) = CosFile::parse_caos(input).expect("Successful parse");
        assert_eq!(
            res,
            CosFile {
                removal_script: None,
                install_script: None,
                event_scripts: vec![
                    EventScriptDefinition {
                        definition: ScriptDefinition {
                            commands: vec![Command::Inst]
                        },
                        family: 4,
                        genus: 0,
                        species: 0,
                        script_number: 34
                    },
                    EventScriptDefinition {
                        definition: ScriptDefinition {
                            commands: vec![Command::Inst]
                        },
                        family: 4,
                        genus: 0,
                        species: 0,
                        script_number: 34
                    },
                ]
            }
        );
    }

    #[test]
    fn test_implict_install_parse() {
        let (_, c) = CosFile::parse(CONTENT).expect("Successful parse");

        assert_eq!(
            c,
            CosFile {
                removal_script: None,
                event_scripts: vec![],
                install_script: Some(ScriptDefinition {
                    commands: vec![
                        Command::Inst,
                        Command::NewSimp {
                            family: IntArg::from_primary(1.into()),
                            genus: IntArg::from_primary(1.into()),
                            species: IntArg::from_primary(114.into()),
                            sprite_file: String::from("blnk").into(),
                            image_count: IntArg::from_primary(1.into()),
                            first_image: IntArg::from_primary(0.into()),
                            plane: IntArg::from_primary(0.into())
                        },
                        Command::Tick {
                            tick_rate: IntArg::from_primary(9.into())
                        },
                        Command::Reps {
                            count: IntArg::from_primary(100.into())
                        },
                        Command::NewSimp {
                            family: IntArg::from_primary(2.into()),
                            genus: IntArg::from_primary(10.into()),
                            species: IntArg::from_primary(37.into()),
                            sprite_file: String::from("graz").into(),
                            image_count: IntArg::from_primary(2.into()),
                            first_image: IntArg::from_primary(218.into()),
                            plane: IntArg::from_primary(3000.into())
                        },
                        Command::Attr {
                            attributes: IntArg::from_primary(192.into())
                        },
                        Command::Elas {
                            elasticity: IntArg::from_primary(0.into())
                        },
                        Command::Setv {
                            var: Variable::Vaxx(0),
                            value: Decimal::Integer(Integer::Rand {
                                value1: Box::new(IntArg::from_primary(0.into())),
                                value2: Box::new(IntArg::from_primary(2.into()))
                            })
                        },
                        Command::Doif {
                            condition: Condition::Simple {
                                cond_type: ConditionType::Eq,
                                lhs: Anything::Variable(Variable::Vaxx(0)),
                                rhs: Anything::Decimal(Decimal::Integer(0.into()))
                            }
                        },
                        Command::Accg {
                            acceleration: FloatArg::from_primary(0.1f32.into())
                        },
                        Command::Elif {
                            condition: Condition::Simple {
                                cond_type: ConditionType::Eq,
                                lhs: Anything::Variable(Variable::Vaxx(0)),
                                rhs: Anything::Decimal(Decimal::Integer(1.into()))
                            }
                        },
                        Command::Accg {
                            acceleration: FloatArg::from_primary(0.3f32.into())
                        },
                        Command::Else,
                        Command::Accg {
                            acceleration: FloatArg::from_primary(0.4f32.into())
                        },
                        Command::Endi,
                        Command::Mvto {
                            x: FloatArg::from_castable(Integer::Rand {
                                value1: Box::new(IntArg::from_primary(217.into())),
                                value2: Box::new(IntArg::from_primary(2787.into()))
                            }),
                            y: FloatArg::from_primary(1840f32.into()),
                        },
                        Command::Perm {
                            permiability: IntArg::from_primary(Integer::Rand {
                                value1: Box::new(IntArg::from_primary(0.into())),
                                value2: Box::new(IntArg::from_primary(70.into()))
                            })
                        },
                        Command::Repe,
                    ]
                })
            }
        );
    }
}
