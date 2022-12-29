use crate::engine::Script;
use crate::{Result, CaosError, ErrorType};
use nom::number::complete::float;
use crate::parser::CaosParseResult;

pub(in super) fn eval_raw(_script: &mut Script, raw: f32) -> Result<f32> {
    Ok(raw)
}

pub(in super) fn eval_stof(_script: &mut Script, str: &str) -> Result<f32> {
    // Beware: we deliberately ignore any remaining string - as according to the CAOS
    // documentation on this behavior.
    let f : CaosParseResult<_, _> = float(str);
    let (_, float_part) = f.map_err(|_| CaosError::new(ErrorType::ParseError))?;
    Ok(float_part)
}

pub(in super) fn eval_tan(_script: &mut Script, theta: f32) -> Result<f32> {
    Ok(theta.to_radians().tan())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::engine::EvaluateCommand;

    use crate::commands::{FloatArg, Float};

    #[test]
    fn test_tan() {
        let fpi = Float::Tan { theta: Box::new(FloatArg::from_primary(180f32.into())) };
        let fpi = fpi.evaluate(&mut Script::new_empty()).expect("Successful evaluation");
        assert_eq!(fpi, 0.00000008742278);
    }

    
    #[test]
    fn test_stof() {
        let fstr = Float::Stof { value: Box::new(String::from("123.43hello_world").into()) };
        let fstr = fstr.evaluate(&mut Script::new_empty()).expect("Successful evaluation");
        assert_eq!(fstr, 123.43);
    }

}