use crate::ast::{IntArg, ScriptDefinition};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ClassifierEnum {
    pub family: Box<IntArg>,
    pub genus: Box<IntArg>,
    pub species: Box<IntArg>,
    pub definition: ScriptDefinition,
}
