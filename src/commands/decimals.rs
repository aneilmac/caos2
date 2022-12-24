use crate::parser::{CaosParsable, CaosParseResult};

use super::{Float, Integer};
use nom::combinator::consumed;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decimal {
    Integer(Integer),
    Float(Float),
}

impl CaosParsable for Decimal {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        // Attempt to consume input as both an int and float and interpret as the one which fits best.
        let f_res = consumed(Float::parse_caos)(input);
        let i_res = consumed(Integer::parse_caos)(input);

        match (f_res, i_res) {
            (Err(..), Err(i_err)) => Err(i_err),
            (Err(..), i @ Ok(..)) => i.map(|(s, (_, i))| (s, Decimal::Integer(i))),
            (f @ Ok(..), Err(..)) => f.map(|(s, (_, f))| (s, Decimal::Float(f))),
            (Ok((f_input, (f_consumed, f))), Ok((i_input, (i_consumed, i)))) => {
                if i_consumed == f_consumed {
                    // If both the float and integer parsers consume the same, we say it's an integer.
                    Ok((i_input, Decimal::Integer(i)))
                } else {
                    // Otherwise we consume as a float.
                    Ok((f_input, Decimal::Float(f)))
                }
            }
        }
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
