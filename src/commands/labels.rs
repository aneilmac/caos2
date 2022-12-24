use crate::parser::{CaosParsable, CaosParseResult};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Label(String);

impl CaosParsable for Label {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        todo!("TODO")
    }
}
