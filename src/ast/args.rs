use super::{Float, Integer, Variable};

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
