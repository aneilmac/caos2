use super::{Agent, ByteString, Decimal, Float, Integer, SString, Variable};
use caos_macros::CaosParsable;

#[derive(Eq, PartialEq, Debug, Clone, CaosParsable)]

pub enum Anything {
    Variable(Variable),
    String(SString),
    Decimal(Decimal),
    ByteString(ByteString),
    Agent(Agent),
}

impl From<Variable> for Anything {
    fn from(v: Variable) -> Self {
        Self::Variable(v)
    }
}

impl From<SString> for Anything {
    fn from(v: SString) -> Self {
        Self::String(v)
    }
}

impl From<Decimal> for Anything {
    fn from(v: Decimal) -> Self {
        Self::Decimal(v)
    }
}

impl From<ByteString> for Anything {
    fn from(v: ByteString) -> Self {
        Self::ByteString(v)
    }
}

impl From<Agent> for Anything {
    fn from(v: Agent) -> Self {
        Self::Agent(v)
    }
}

impl From<Integer> for Anything {
    fn from(i: Integer) -> Self {
        Self::Decimal(i.into())
    }
}

impl From<i32> for Anything {
    fn from(i: i32) -> Self {
        Self::Decimal(i.into())
    }
}

impl From<Float> for Anything {
    fn from(f: Float) -> Self {
        Self::Decimal(f.into())
    }
}

impl From<f32> for Anything {
    fn from(f: f32) -> Self {
        Self::Decimal(f.into())
    }
}
