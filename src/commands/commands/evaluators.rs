use crate::engine::{ScriptRefMut, Variadic};
use crate::{CaosError, ErrorType, Result};

pub(super) fn eval_orrv(script: &mut ScriptRefMut<'_>, v: Variadic, x: i32) -> Result<()> {
    todo!()
    // match v {
    //     Variadic::Integer(i) => { *i |= x; Ok(()) }
    //     _ => Err(CaosError::new(ErrorType::TypeMismatch))
    // }
}
