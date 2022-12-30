use nom::{combinator::{opt, consumed, all_consuming}, bytes::complete::tag_no_case, multi::separated_list0, error::{VerboseError, ParseError, FromExternalError}, Finish};

use crate::{parser::{ScriptDefinition, Script, EventScriptDefinition, CaosParseResult, CaosParsable}, CaosError, ErrorType};
use super::{caos_skippable1, caos_skippable0};

pub struct CosFile {
    pub install_script: Option<ScriptDefinition>,
    pub removal_script: Option<ScriptDefinition>,
    pub event_scripts: Vec<EventScriptDefinition>,
}

impl CosFile {
    pub fn parse(input: &str) ->  Result<(&str, Self), VerboseError<&str>> {
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
        let (input,  implicit_install_script) = opt(Script::parse_definition)(input)?;

        // Weird corner case of old CAOS parser:
        // Implicit installation scripts may still be closed with an `endm` tag,
        // and in-fact we can have a dangling `endm` tag at the very start of the script.
        let (input, _) = opt(tag_no_case(Script::ENDM_TAG))(input)?;

        let (input, scripts) = separated_list0(caos_skippable1, consumed(Script::parse_caos))(input)?;

        let mut install_script: Option<ScriptDefinition> = None;
        let mut removal_script: Option<ScriptDefinition> = None;
        let mut event_scripts : Vec<EventScriptDefinition> = vec![];

        if let Some(i) = implicit_install_script {
            install_script = Some(i);
        }

        for (consumed, script) in scripts {
            match script {
                Script::Install(i) => if install_script.is_none() {
                    install_script = Some(i);
                } else {
                    return Err(nom::Err::Failure(VerboseError::from_external_error(consumed, 
                        nom::error::ErrorKind::Complete, 
                        CaosError::new(ErrorType::TooManyInstallScripts)
                    )));
                },
                Script::Removal(r) => if removal_script.is_none() {
                    removal_script = Some(r);
                } else {
                    return Err(nom::Err::Failure(VerboseError::from_external_error(consumed, 
                        nom::error::ErrorKind::Complete, 
                        CaosError::new(ErrorType::TooManyRemovalScripts)
                    )));
                }
                Script::Event(e) => { event_scripts.push(e); }
            }
        }

        Ok((input, Self { install_script, removal_script, event_scripts } ))
    }
}