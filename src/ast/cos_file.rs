use crate::ast::Script;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct CosFile {
    pub scripts: Vec<Script>,
}
