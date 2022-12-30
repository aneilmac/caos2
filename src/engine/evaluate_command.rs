use crate::engine::ScriptRefMut;
use crate::Result;

use super::AgentRef;

/// Used to evaluate a Command structure and returns  its given `ReturnType`.
/// E.g. an `IntArg` will be evaluated and produce an `i32`,
/// a `Condition` will be evaluated and produce a `bool`.
pub(crate) trait EvaluateCommand {
    type ReturnType;

    fn evaluate(&self, script: &mut ScriptRefMut<'_>) -> Result<Self::ReturnType>;
}

impl EvaluateCommand for u8 {
    type ReturnType = u8;

    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> Result<Self::ReturnType> {
        Ok(*self)
    }
}

impl EvaluateCommand for f32 {
    type ReturnType = f32;

    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> Result<Self::ReturnType> {
        Ok(*self)
    }
}

impl EvaluateCommand for i32 {
    type ReturnType = i32;

    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> Result<Self::ReturnType> {
        Ok(*self)
    }
}

impl EvaluateCommand for bool {
    type ReturnType = bool;

    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> Result<Self::ReturnType> {
        Ok(*self)
    }
}

impl<'a> EvaluateCommand for &'a String {
    type ReturnType = &'a String;

    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> Result<Self::ReturnType> {
        Ok(*self)
    }
}

impl<'a> EvaluateCommand for &'a Vec<u8> {
    type ReturnType = &'a Vec<u8>;

    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> Result<Self::ReturnType> {
        Ok(*self)
    }
}

impl<'a> EvaluateCommand for &'a AgentRef {
    type ReturnType = &'a AgentRef;

    fn evaluate(&self, _script: &mut ScriptRefMut<'_>) -> Result<Self::ReturnType> {
        Ok(*self)
    }
}
