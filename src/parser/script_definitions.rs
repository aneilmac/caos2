use crate::commands::{Command, LiteralInt};
use crate::parser::{caos_skippable1, CaosParsable, CaosParseResult};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::{eof, map, opt};
use nom::multi::separated_list0;
use nom::sequence::tuple;

use super::caos_skippable0;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EventScriptDefinition {
    pub definition: ScriptDefinition,
    pub family: i32,
    pub genus: i32,
    pub species: i32,
    pub script_number: i32,
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct ScriptDefinition {
    pub commands: Vec<Command>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Script {
    Install(ScriptDefinition),
    Removal(ScriptDefinition),
    Event(EventScriptDefinition),
}

impl Script {
    pub const ISCR_TAG: &str = "iscr";
    pub const SCRP_TAG: &str = "scrp";
    pub const RSCR_TAG: &str = "rscr";
    pub const ENDM_TAG: &str = "endm";

    pub fn definition(&self) -> &ScriptDefinition {
        match self {
            Self::Install(definition) => definition,
            Self::Removal(definition) => definition,
            Self::Event(event_definition) => &event_definition.definition,
        }
    }

    pub fn definition_mut(&mut self) -> &mut ScriptDefinition {
        match self {
            Self::Install(definition) => definition,
            Self::Removal(definition) => definition,
            Self::Event(event_definition) => &mut event_definition.definition,
        }
    }

    pub fn is_install(&self) -> bool {
        match self {
            Self::Install(..) => true,
            _ => false,
        }
    }

    pub fn is_removal(&self) -> bool {
        match self {
            Self::Removal(..) => true,
            _ => false,
        }
    }

    pub fn is_event(&self) -> bool {
        match self {
            Self::Event(..) => true,
            _ => false,
        }
    }

    // ISCR is very gnarly and at the time of writing.
    // 1. There may only be one in a *.cos file.
    // 2. Implicitly defined at the top of the script if not provided.
    // 3. (Looks like implicitly defined block is ignored when `iscr` provided?)
    // 4. Does not require an `ENDM`, can be implicitly ended by `RSCR`, `SCRP` or
    //    the end of the file.
    fn parse_install_script(input: &str) -> CaosParseResult<&str, Self> {
        map(
            |input| Self::parse_inst_script(input, Self::ISCR_TAG),
            Self::Install,
        )(input)
    }

    // RSCR is not as bad as ISCR, but still has lots of "fun".
    // 1. There may only be one in a *.cos file.
    // 2. Does not require an `ENDM`, can be implicitly ended by `ISCR`, `SCRP` or
    //    the end of the file.
    fn parse_removal_script(input: &str) -> CaosParseResult<&str, Self> {
        map(
            |input| Self::parse_inst_script(input, Self::RSCR_TAG),
            Self::Removal,
        )(input)
    }

    // SCRP is a sensible tag.
    // 1. Can be multiple.
    // 2. Must always have a closing ENDM.
    fn parse_event(input: &str) -> CaosParseResult<&str, Self> {
        let (input, _) = tag_no_case(Self::SCRP_TAG)(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, family) = LiteralInt::parse_caos(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, genus) = LiteralInt::parse_caos(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, species) = LiteralInt::parse_caos(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, script_number) = LiteralInt::parse_caos(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, definition) = Self::parse_definition(input)?;

        let input = if definition.commands.len() > 0 {
            let (input, _) = caos_skippable1(input)?;
            input
        } else {
            input
        };

        let (input, _) = tag_no_case(Self::ENDM_TAG)(input)?;

        Ok((
            input,
            Self::Event(EventScriptDefinition {
                family: family.into(),
                genus: genus.into(),
                species: species.into(),
                script_number: script_number.into(),
                definition,
            }),
        ))
    }

    fn parse_inst_script<'a>(
        input: &'a str,
        tag: &str,
    ) -> CaosParseResult<&'a str, ScriptDefinition> {
        let (input, _) = tag_no_case(tag)(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, definition) = Self::parse_definition(input)?;

        // If at EOF, do an early return.
        if let (input, Some(_)) = opt(tuple((caos_skippable0, eof)))(input)? {
            return Ok((input, definition));
        }

        // Optional ENDM at end of block.
        if definition.commands.len() > 0 {
            let (input, _) = opt(tuple((caos_skippable1, tag_no_case(Self::ENDM_TAG))))(input)?;
            Ok((input, definition))
        } else {
            let (input, _) = opt(tag_no_case(Self::ENDM_TAG))(input)?;
            Ok((input, definition))
        }
    }

    pub(crate) fn parse_definition(input: &str) -> CaosParseResult<&str, ScriptDefinition> {
        let (input, commands) = separated_list0(caos_skippable1, Command::parse_caos)(input)?;
        Ok((input, ScriptDefinition { commands }))
    }
}

impl CaosParsable for Script {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self> {
        alt((
            Self::parse_install_script,
            Self::parse_event,
            Self::parse_removal_script,
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::{Agent, FloatArg, IntArg};

    use super::*;

    const EVENT_SCRIPT: &str = r#"scrp 2 18 14 12
** egg eat script
    stim writ from 80 1
    kill ownr
endm"#;

    #[test]
    fn test_parse_definition() {
        let input = "inst **moisture monitor\nnew: simp 1 1 114 \"blnk\" 1 0 0\ntick 9";
        let (_, def) = Script::parse_definition(input).expect("Successful parse");

        assert_eq!(
            def.commands,
            vec![
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
            ]
        );
    }

    #[test]
    fn test_parse_install_empty() {
        let input = "iscr\nendm";
        let (_, script) = Script::parse_caos(input).expect("Successful parse");
        assert!(script.is_install());
        assert!(script.definition().commands.is_empty());
    }

    #[test]
    fn test_parse_install_empty_no_endm() {
        let input = "iscr\n";
        let (_, script) = Script::parse_caos(input).expect("Successful parse");
        assert!(script.is_install());
        assert!(script.definition().commands.is_empty());
    }

    #[test]
    fn test_parse_install_empty_with_rscr() {
        let input = "iscr\nrscr";
        let (_, script) = Script::parse_caos(input).expect("Successful parse");
        assert!(script.is_install());
        assert!(script.definition().commands.is_empty());
    }

    #[test]
    fn test_parse_install_nonempty() {
        let input = "iscr\ninst\nendm";
        let (_, script) = Script::parse_caos(input).expect("Successful parse");
        assert!(script.is_install());
        assert_eq!(script.definition().commands, vec![Command::Inst]);
    }

    #[test]
    fn test_parse_install_nonempty_with_rscr() {
        let input = "iscr\ninst\nrscr";
        let (_, script) = Script::parse_caos(input).expect("Successful parse");
        assert!(script.is_install());
        assert_eq!(script.definition().commands, vec![Command::Inst]);
    }

    #[test]
    fn test_parse_install_no_spaces() {
        let input = "iscrendm";
        Script::parse_caos(input).expect_err("Failed parse");
    }

    #[test]
    fn test_parse_install_nonempty_no_endm() {
        let input = "iscr\ninst";
        let (_, script) = Script::parse_caos(input).expect("Successful parse");
        assert!(script.is_install());
        assert_eq!(script.definition().commands, vec![Command::Inst]);
    }

    #[test]
    fn test_parse_install_nonempty_no_endm_with_newline() {
        let input = "iscr\ninst\n";
        let (_, script) = Script::parse_caos(input).expect("Successful parse");
        assert!(script.is_install());
        assert_eq!(script.definition().commands, vec![Command::Inst]);
    }

    #[test]
    fn test_event_script() {
        let input = "scrp 4 0 0 34\ninst\nendm";
        if let (_, Script::Event(s)) = Script::parse_caos(input).expect("Successful parse") {
            assert_eq!(s.definition.commands, vec![Command::Inst]);
            assert_eq!(s.family, 4);
            assert_eq!(s.genus, 0);
            assert_eq!(s.species, 0);
            assert_eq!(s.script_number, 34)
        } else {
            panic!("Not an event");
        }
    }

    #[test]
    fn test_event_script_no_endm() {
        let input = "scrp 4 0 0 34\ninst";
        Script::parse_caos(input).expect_err("No endm");
    }

    #[test]
    fn test_known_event_script() {
        let (s, i) = Script::parse_caos(EVENT_SCRIPT).expect("Successful parse");
        assert_eq!(s, "");
        assert_eq!(
            i,
            Script::Event(EventScriptDefinition {
                definition: ScriptDefinition {
                    commands: vec![
                        Command::StimWrit {
                            creature: Agent::From,
                            stimulus: IntArg::from_primary(80.into()),
                            strength: FloatArg::from_primary(1f32.into())
                        },
                        Command::Kill { agent: Agent::Ownr }
                    ]
                },
                family: 2,
                genus: 18,
                species: 14,
                script_number: 12
            })
        );
    }
}
