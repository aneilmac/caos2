use crate::engine::ScriptRefMut;
use crate::parser::CaosParseResult;
use crate::{CaosError, ErrorType, Result};
use nom::number::complete::float;

pub(super) fn eval_raw(_script: &mut ScriptRefMut<'_>, raw: f32) -> Result<f32> {
    Ok(raw)
}

pub(super) fn eval_stof(_script: &mut ScriptRefMut<'_>, str: String) -> Result<f32> {
    // Beware: we deliberately ignore any remaining string - as according to the CAOS
    // documentation on this behavior.
    let f: CaosParseResult<_, _> = float(&*str);
    let (_, float_part) = f.map_err(|_| CaosError::new(ErrorType::ParseError))?;
    Ok(float_part)
}

pub(super) fn eval_tan(_script: &mut ScriptRefMut<'_>, theta: f32) -> Result<f32> {
    Ok(theta.to_radians().tan())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::engine::EvaluateCommand;

    use crate::commands::{Float, FloatArg};

    use crate::engine::Script;

    #[test]
    fn test_tan() {
        let fpi = Float::Tan {
            theta: Box::new(FloatArg::from_primary(180f32.into())),
        };
        let fpi = fpi
            .evaluate(&mut ScriptRefMut::new(&mut Script::new_empty()))
            .expect("Successful evaluation");
        assert_eq!(fpi, 0.00000008742278);
    }

    #[test]
    fn test_stof() {
        let fstr = Float::Stof {
            value: Box::new(String::from("123.43hello_world").into()),
        };
        let fstr = fstr
            .evaluate(&mut ScriptRefMut::new(&mut Script::new_empty()))
            .expect("Successful evaluation");
        assert_eq!(fstr, 123.43);
    }
}
