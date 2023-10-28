use super::{Agent, Decimal, Float, Integer, SString, Variable};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum AgentArg {
    Agent(Agent),
    Variable(Variable),
}

impl From<Agent> for AgentArg {
    fn from(value: Agent) -> Self {
        Self::Agent(value)
    }
}

impl From<Variable> for AgentArg {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum SStringArg {
    String(SString),
    Variable(Variable),
}

impl From<SString> for SStringArg {
    fn from(value: SString) -> Self {
        Self::String(value)
    }
}

impl From<String> for SStringArg {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}

impl From<Variable> for SStringArg {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DecimalArg {
    Decimal(Decimal),
    Variable(Variable),
}

impl From<Decimal> for DecimalArg {
    fn from(value: Decimal) -> Self {
        Self::Decimal(value)
    }
}

impl From<Variable> for DecimalArg {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

impl From<Integer> for DecimalArg {
    fn from(value: Integer) -> Self {
        Self::Decimal(value.into())
    }
}

impl From<i32> for DecimalArg {
    fn from(value: i32) -> Self {
        Self::Decimal(value.into())
    }
}

impl From<Float> for DecimalArg {
    fn from(value: Float) -> Self {
        Self::Decimal(value.into())
    }
}

impl From<f32> for DecimalArg {
    fn from(value: f32) -> Self {
        Self::Decimal(value.into())
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum IntArg {
    Primary(Integer),
    Castable(Float),
    Variable(Variable),
}

impl From<Integer> for IntArg {
    fn from(value: Integer) -> Self {
        Self::Primary(value)
    }
}

impl From<i32> for IntArg {
    fn from(value: i32) -> Self {
        Self::Primary(value.into())
    }
}

impl From<Float> for IntArg {
    fn from(value: Float) -> Self {
        Self::Castable(value)
    }
}

impl From<f32> for IntArg {
    fn from(value: f32) -> Self {
        Self::Castable(value.into())
    }
}

impl From<Variable> for IntArg {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum FloatArg {
    Primary(Float),
    Castable(Integer),
    Variable(Variable),
}

impl From<Float> for FloatArg {
    fn from(value: Float) -> Self {
        Self::Primary(value)
    }
}

impl From<f32> for FloatArg {
    fn from(value: f32) -> Self {
        Self::Primary(value.into())
    }
}

impl From<Integer> for FloatArg {
    fn from(value: Integer) -> Self {
        Self::Castable(value)
    }
}

impl From<i32> for FloatArg {
    fn from(value: i32) -> Self {
        Self::Castable(value.into())
    }
}

impl From<Variable> for FloatArg {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}
