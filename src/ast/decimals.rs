use super::{Float, Integer, Variable};
use caos_macros::CaosParsable;

#[derive(CaosParsable, Debug, PartialEq, Eq, Clone)]
pub enum Decimal {
    Integer(Integer),
    Float(Float),
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
