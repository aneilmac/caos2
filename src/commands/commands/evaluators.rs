use crate::{Result, CaosError, ErrorType};
use crate::engine::{Variadic, ScriptRefMut};

pub(super) fn eval_orrv(script: &mut ScriptRefMut<'_>, v: Variadic, x: i32) -> Result<()> {
   todo!()
    // match v {
    //     Variadic::Integer(i) => { *i |= x; Ok(()) }
    //     _ => Err(CaosError::new(ErrorType::TypeMismatch))
    // }
}