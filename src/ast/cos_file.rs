use crate::ast::{EventScriptDefinition, ScriptDefinition};

use caos_macros::CaosParsable;

#[derive(CaosParsable, Debug, Eq, PartialEq)]
pub struct CosFile {
    pub install_script: Option<ScriptDefinition>,
    pub removal_script: Option<ScriptDefinition>,
    pub event_scripts: Vec<EventScriptDefinition>,
}
