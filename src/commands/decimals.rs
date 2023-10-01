use crate::{
    engine::{EvaluateCommand, ScriptRefMut},
    parser::{CaosParsable, CaosParseResult},
};

use super::{Float, Integer, LiteralF32, LiteralInt, Variable};
use nom::{
    branch::alt,
    combinator::{consumed, map},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decimal {
    Variable(Variable),
    Integer(Integer),
    Float(Float),
}

impl CaosParsable for Decimal {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        // Attempt to consume input as both an int and float and interpret as the one which fits best.
        let f_res = consumed(LiteralF32::parse_caos)(input);
        let i_res = consumed(LiteralInt::parse_caos)(input);

        match (f_res, i_res) {
            (Err(..), i @ Ok(..)) => i.map(|(s, (_, i))| (s, Decimal::Integer(i.into()))),
            (f @ Ok(..), Err(..)) => f.map(|(s, (_, f))| (s, Decimal::Float(f.into()))),
            (Ok((f_input, (f_consumed, f))), Ok((i_input, (i_consumed, i)))) => {
                if i_consumed == f_consumed {
                    // If both the float and integer parsers consume the same, we say it's an integer.
                    Ok((i_input, Decimal::Integer(i.into())))
                } else {
                    // Otherwise we consume as a float.
                    Ok((f_input, Decimal::Float(f.into())))
                }
            }
            (Err(..), Err(..)) => {
                // TODO This might not be good for variable types
                alt((
                    map(Variable::parse_caos, Decimal::Variable),
                    map(Float::parse_caos, Decimal::Float),
                    map(Integer::parse_caos, Decimal::Integer),
                ))(input)
            }
        }
    }
}


impl From<Integer> for Decimal {
    fn from(i: Integer) -> Self {
        Self::Integer(i)
    }
}

impl From<Float> for Decimal {
    fn from(f: Float) -> Self {
        Self::Float(f)
    }
}

impl From<Variable> for Decimal {
    fn from(v: Variable) -> Self {
        Self::Variable(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_int() {
        let (_, res) = Decimal::parse_caos("3").expect("Valid Anything");
        assert_eq!(res, Decimal::Integer(3.into()));
    }

    #[test]
    fn test_rounded_float() {
        let (_, res) = Decimal::parse_caos("3.0").expect("Valid Anything");
        assert_eq!(res, Decimal::Float(3.0.into()));
    }

    #[test]
    fn test_literal_float() {
        let (_, res) = Decimal::parse_caos("3.1").expect("Valid Anything");
        assert_eq!(res, Decimal::Float(3.1.into()));
    }
}
