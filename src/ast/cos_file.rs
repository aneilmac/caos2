use crate::ast::{EventScriptDefinition, Script};

use caos_macros::CaosParsable;

#[derive(CaosParsable, Debug, Eq, PartialEq, Default)]
pub struct CosFile {
    pub scripts: Vec<Script>,
}
