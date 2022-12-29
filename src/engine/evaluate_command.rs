use crate::engine::Script;
use crate::Result;

pub(in crate) trait EvaluateCommand {
    type ReturnType;

    fn evaluate(&self, script: &mut Script) -> Result<Self::ReturnType>;
}
