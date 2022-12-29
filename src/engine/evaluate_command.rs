use crate::engine::Script;
use crate::Result;

pub(in crate) trait EvaluateCommand {
    type ReturnType;

    fn evaluate(&self, script: &mut Script) -> Result<Self::ReturnType>;
}

impl EvaluateCommand for u8 {
    type ReturnType = u8;

    fn evaluate(&self, _script: &mut Script) -> Result<Self::ReturnType> {
        Ok(*self)
    }
}