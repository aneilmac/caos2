use crate::engine::ScriptRefMut;
use crate::Result;

pub(super) fn eval_raw(_script: &mut ScriptRefMut<'_>, raw: &String) -> Result<String> {
    Ok(raw.clone())
}
