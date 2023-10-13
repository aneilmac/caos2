use super::{Float, Integer, Variable};

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl From<i32> for Decimal {
    fn from(i: i32) -> Self {
        Self::Integer(i.into())
    }
}

impl From<f32> for Decimal {
    fn from(f: f32) -> Self {
        Self::Float(f.into())
    }
}
