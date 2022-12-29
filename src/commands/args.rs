use nom::branch::alt;
use nom::combinator::map;

use crate::commands::{Float, Integer, Variable};
use crate::{CaosError, ErrorType};

use crate::engine::{EvaluateCommand, Variadic};
use crate::parser::{CaosParsable, CaosParseResult};

pub type IntArg = DecimalArg<Integer, Float>;

pub type FloatArg = DecimalArg<Float, Integer>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DecimalArg<P, C> {
    Primary(P),
    Castable(C),
    Variable(Variable),
}

impl<P, C> DecimalArg<P, C> {
    pub fn from_primary(p: P) -> Self {
        Self::Primary(p)
    }

    pub fn from_castable(c: C) -> Self {
        Self::Castable(c)
    }

    pub fn from_variable(v: Variable) -> Self {
        Self::Variable(v)
    }
}

impl<P: CaosParsable, C> From<P> for DecimalArg<P, C> {
    fn from(p: P) -> Self {
        Self::Primary(p)
    }
}

impl<P, C> CaosParsable for DecimalArg<P, C>
where
    P: CaosParsable,
    C: CaosParsable,
{
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            map(Variable::parse_caos, Self::Variable),
            map(P::parse_caos, Self::Primary),
            map(C::parse_caos, Self::Castable),
        ))(input)
    }
}

impl<P, C> EvaluateCommand for DecimalArg<P, C>
where
    P: EvaluateCommand,
    C: EvaluateCommand,
    <P as EvaluateCommand>::ReturnType:
        Castable<<C as EvaluateCommand>::ReturnType> + Castable<f32> + Castable<i32>,
{
    type ReturnType = <P as EvaluateCommand>::ReturnType;
    fn evaluate(&self, script: &mut crate::engine::Script) -> crate::Result<Self::ReturnType> {
        match self {
            Self::Primary(p) => p.evaluate(script),
            Self::Castable(c) => c.evaluate(script).map(|c| Self::ReturnType::cast(c)),
            Self::Variable(v) => v.evaluate(script).and_then(|v| match v {
                Variadic::Float(f) => Ok(Self::ReturnType::cast(f)),
                Variadic::Integer(i) => Ok(Self::ReturnType::cast(i)),
                _ => Err(CaosError::new(ErrorType::DecimalConversionFailure)),
            }),
        }
    }
}

trait Castable<T> {
    fn cast(t: T) -> Self;
}

impl<T> Castable<T> for T {
    fn cast(t: T) -> Self {
        t
    }
}

impl Castable<f32> for i32 {
    fn cast(f: f32) -> Self {
        f as Self
    }
}

impl Castable<i32> for f32 {
    fn cast(i: i32) -> Self {
        i as Self
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::{Command, SString, Variable};

    use super::*;

    #[test]
    fn test_integer_cast() {
        let (_, res) = Command::parse_caos("mvto rand 217 2787 1840").expect("Valid command");
        assert_eq!(
            res,
            Command::Mvto {
                y: FloatArg::from_primary(1840f32.into()),
                x: FloatArg::from_castable(Integer::Rand {
                    value1: Box::new(IntArg::from_primary(217.into())),
                    value2: Box::new(IntArg::from_primary(2787.into()))
                })
            }
        );
    }

    #[test]
    fn test_float_cast() {
        let (_, res) =
            Command::parse_caos("snap va00 posx posy 119 139 100").expect("Valid command");
        assert_eq!(
            res,
            Command::Snap {
                filename: SString::Variable(Box::new(Variable::Vaxx(0))),
                x_centre: IntArg::from_castable(Float::Posx),
                y_centre: IntArg::from_castable(Float::Posy),
                width: IntArg::from_primary(119.into()),
                height: IntArg::from_primary(139.into()),
                zoom_factor: IntArg::from_primary(100.into())
            }
        );
    }

    #[test]
    fn test_variable_float() {
        let (_, res) = FloatArg::parse_caos("velx").expect("Valid float");
        assert_eq!(res, FloatArg::from_variable(Variable::Velx));
    }
}
