use std::convert::Infallible;

use crate::{CaosError, ErrorType};

use super::{
    Agent, AgentArg, ByteString, Decimal, DecimalArg, Float, FloatArg, IntArg, Integer, SString,
    SStringArg, Variable,
};

#[derive(Eq, PartialEq, Debug, Clone)]

pub enum Anything {
    Variable(Variable),
    String(SString),
    Decimal(Decimal),
    ByteString(ByteString),
    Agent(Agent),
}

impl Anything {
    pub fn variable_or_else<E, F>(self, err: F) -> Result<Variable, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Anything::Variable(v) => Ok(v),
            _ => Err(err()),
        }
    }

    pub fn variable_or<E>(self, e: E) -> Result<Variable, E> {
        self.variable_or_else(|| e)
    }

    pub fn string_arg_or_else<E, F>(self, err: F) -> Result<SStringArg, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Anything::Variable(v) => Ok(SStringArg::Variable(v)),
            Anything::String(s) => Ok(SStringArg::String(s)),
            _ => Err(err()),
        }
    }

    pub fn string_arg_or<E>(self, e: E) -> Result<SStringArg, E> {
        self.string_arg_or_else(|| e)
    }

    pub fn decimal_arg_or_else<E, F>(self, err: F) -> Result<DecimalArg, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Anything::Variable(v) => Ok(DecimalArg::Variable(v)),
            Anything::Decimal(d) => Ok(DecimalArg::Decimal(d)),
            _ => Err(err()),
        }
    }

    pub fn decimal_or<E>(self, e: E) -> Result<DecimalArg, E> {
        self.decimal_arg_or_else(|| e)
    }

    pub fn int_arg_or_else<E, F>(self, err: F) -> Result<IntArg, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Anything::Variable(v) => Ok(IntArg::Variable(v)),
            Anything::Decimal(Decimal::Integer(i)) => Ok(IntArg::Primary(i)),
            Anything::Decimal(Decimal::Float(f)) => Ok(IntArg::Castable(f)),
            _ => Err(err()),
        }
    }

    pub fn int_arg_or<E>(self, e: E) -> Result<IntArg, E> {
        self.int_arg_or_else(|| e)
    }

    pub fn float_arg_or_else<E, F>(self, err: F) -> Result<FloatArg, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Anything::Variable(v) => Ok(FloatArg::Variable(v)),
            Anything::Decimal(Decimal::Integer(i)) => Ok(FloatArg::Castable(i)),
            Anything::Decimal(Decimal::Float(f)) => Ok(FloatArg::Primary(f)),
            _ => Err(err()),
        }
    }

    pub fn float_or<E>(self, e: E) -> Result<FloatArg, E> {
        self.float_arg_or_else(|| e)
    }

    pub fn bytestring_or_else<E, F>(self, err: F) -> Result<ByteString, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Anything::ByteString(bs) => Ok(bs),
            _ => Err(err()),
        }
    }

    pub fn bytestring_or<E>(self, e: E) -> Result<ByteString, E> {
        self.bytestring_or_else(|| e)
    }

    pub fn agent_arg_or_else<E, F>(self, err: F) -> Result<AgentArg, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Anything::Variable(bs) => Ok(AgentArg::Variable(bs)),
            Anything::Agent(bs) => Ok(AgentArg::Agent(bs)),
            _ => Err(err()),
        }
    }

    pub fn agent_arg_or<E>(self, e: E) -> Result<AgentArg, E> {
        self.agent_arg_or_else(|| e)
    }
}

impl From<Variable> for Anything {
    fn from(v: Variable) -> Self {
        Self::Variable(v)
    }
}

impl TryFrom<Anything> for Variable {
    type Error = CaosError;
    fn try_from(value: Anything) -> Result<Self, Self::Error> {
        value.variable_or_else(|| {
            CaosError::new(
                ErrorType::CastError  { line_col: None },
                String::from("Could not cast to Variable"),
            )
        })
    }
}

impl From<SString> for Anything {
    fn from(v: SString) -> Self {
        Self::String(v)
    }
}

impl From<String> for Anything {
    fn from(v: String) -> Self {
        Self::String(v.into())
    }
}

impl TryFrom<Anything> for SStringArg {
    type Error = CaosError;
    fn try_from(value: Anything) -> Result<Self, Self::Error> {
        value.string_arg_or_else(|| {
            CaosError::new(
                ErrorType::CastError { line_col: None },
                String::from("Could not cast to string"),
            )
        })
    }
}

impl From<Decimal> for Anything {
    fn from(v: Decimal) -> Self {
        Self::Decimal(v)
    }
}

impl TryFrom<Anything> for DecimalArg {
    type Error = CaosError;
    fn try_from(value: Anything) -> Result<Self, Self::Error> {
        value.decimal_arg_or_else(|| {
            CaosError::new(
                ErrorType::CastError  { line_col: None },
                String::from("Could not cast to decimal"),
            )
        })
    }
}

impl From<ByteString> for Anything {
    fn from(v: ByteString) -> Self {
        Self::ByteString(v)
    }
}

impl TryFrom<Anything> for ByteString {
    type Error = CaosError;
    fn try_from(value: Anything) -> Result<Self, Self::Error> {
        value.bytestring_or_else(|| {
            CaosError::new(
                ErrorType::CastError  { line_col: None },
                String::from("Could not cast to decimal"),
            )
        })
    }
}

impl From<Agent> for Anything {
    fn from(v: Agent) -> Self {
        Self::Agent(v)
    }
}

impl TryFrom<Anything> for AgentArg {
    type Error = CaosError;
    fn try_from(value: Anything) -> Result<Self, Self::Error> {
        value.agent_arg_or_else(|| {
            CaosError::new(
                ErrorType::CastError  { line_col: None },
                String::from("Could not cast to agent"),
            )
        })
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

impl TryFrom<Anything> for IntArg {
    type Error = CaosError;
    fn try_from(value: Anything) -> Result<Self, Self::Error> {
        value.int_arg_or_else(|| {
            CaosError::new(ErrorType::CastError  { line_col: None }, String::from("Could not cast to int"))
        })
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

impl TryFrom<Anything> for FloatArg {
    type Error = CaosError;
    fn try_from(value: Anything) -> Result<Self, Self::Error> {
        value.float_arg_or_else(|| {
            CaosError::new(
                ErrorType::CastError  { line_col: None },
                String::from("Could not cast to float"),
            )
        })
    }
}
