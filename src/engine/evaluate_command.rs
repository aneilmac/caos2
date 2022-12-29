use crate::engine::Script;
use crate::Result;

use super::AgentRef;

pub(crate) trait EvaluateCommand {
    type ReturnType;

    fn evaluate(&self, script: &mut Script) -> Result<Self::ReturnType>;
}

impl EvaluateCommand for u8 {
    type ReturnType = u8;

    fn evaluate(&self, _script: &mut Script) -> Result<Self::ReturnType> {
        Ok(*self)
    }
}

impl EvaluateCommand for String {
    type ReturnType = String;

    fn evaluate(&self, _script: &mut Script) -> Result<Self::ReturnType> {
        Ok(self.clone())
    }
}

/// Helper trait to choose whether to pass an argument
/// by ref or by copy. Is only used to neaten up the 
/// signatures of the evaluate methods. For example:
/// we can take arguments by `bool` as opposed to by `&bool`.
pub(in crate) trait EvaluateCommandDeref {
    type ReturnType;

    fn ederef(self) -> Self::ReturnType;
}

impl<'a> EvaluateCommandDeref for &'a bool {
    type ReturnType = bool;
    fn ederef(self) -> Self::ReturnType {
        *self
    }
}

impl<'a> EvaluateCommandDeref for &'a u8 {
    type ReturnType = u8;
    fn ederef(self) -> Self::ReturnType {
        *self
    }
}

impl<'a> EvaluateCommandDeref for &'a i32 {
    type ReturnType = i32;
    fn ederef(self) -> Self::ReturnType {
        *self
    }
}

impl<'a> EvaluateCommandDeref for &'a f32 {
    type ReturnType = f32;
    fn ederef(self) -> Self::ReturnType {
        *self
    }
}

impl<'a> EvaluateCommandDeref for &'a String {
    type ReturnType = &'a str;

    fn ederef(self) -> Self::ReturnType {
        self
    }
}

impl<'a> EvaluateCommandDeref for &'a Vec<u8> {
    type ReturnType = &'a [u8];

    fn ederef(self) -> Self::ReturnType {
        self
    }
}

impl<'a> EvaluateCommandDeref for &'a crate::engine::Variadic {
    type ReturnType = &'a crate::engine::Variadic;

    fn ederef(self) -> Self::ReturnType {
        self
    }
}

