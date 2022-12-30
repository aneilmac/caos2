use crate::engine::{EvaluateCommand, Script, ScriptRefMut};
use crate::parser::{CaosParsable, CaosParseResult};
use crate::Result;
use nom::combinator::map;
use nom::number::complete::float;

/// We want strict equality for our tokens, so we wrap the f32 up and compare it bitwise to ensure
/// "true" equality for token parsing.
#[derive(Debug, Clone)]
pub struct LiteralF32(pub f32);

impl EvaluateCommand for LiteralF32 {
    type ReturnType = f32;
    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> Result<Self::ReturnType> {
        Ok(self.0)
    }
}

impl PartialEq for LiteralF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for LiteralF32 {}

impl From<f32> for LiteralF32 {
    fn from(f: f32) -> Self {
        LiteralF32(f)
    }
}

impl From<LiteralF32> for f32 {
    fn from(f: LiteralF32) -> Self {
        f.0
    }
}

impl CaosParsable for LiteralF32 {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        map(float, LiteralF32)(input)
    }
}
