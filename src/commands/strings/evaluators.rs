use crate::Result;
use crate::engine::Script;

pub(in super) fn eval_raw(_script: &mut Script, raw: &str) -> Result<String> {
    Ok(raw.to_owned())
}