use crate::engine::AgentRef;
use crate::{CaosError, ErrorType, Result};

#[derive(Debug, Clone)]
pub enum Variadic {
    String(String),
    Integer(i32),
    Float(f32),
    Agent(AgentRef),
    ByteString(Vec<u8>),
}

impl Variadic {
    pub fn is_string(&self) -> bool {
        match self {
            Self::String(..) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Self::Integer(..) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Self::Float(..) => true,
            _ => false,
        }
    }

    pub fn is_agent(&self) -> bool {
        match self {
            Self::Agent(..) => true,
            _ => false,
        }
    }

    pub fn is_byte_string(&self) -> bool {
        match self {
            Self::ByteString(..) => true,
            _ => false,
        }
    }

    pub fn is_decimal(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    pub(crate) fn eq(&self, other: &Self) -> Result<bool> {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => Ok(l0 == r0),
            (Self::Integer(l0), Self::Integer(r0)) => Ok(l0 == r0),
            (Self::Float(l0), Self::Float(r0)) => Ok(l0 == r0),
            (Self::Agent(l0), Self::Agent(r0)) => Ok(l0 == r0),
            (Self::ByteString(l0), Self::ByteString(r0)) => Ok(l0 == r0),
            (Self::Integer(l0), Self::Float(r0)) => Ok(*l0 as f32 == *r0),
            (Self::Float(l0), Self::Integer(r0)) => Ok(*l0 == *r0 as f32),
            _ => Err(CaosError::new(ErrorType::TypeMismatch)),
        }
    }

    pub(crate) fn partial_cmp(&self, other: &Self) -> Result<Option<std::cmp::Ordering>> {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => Ok(l0.partial_cmp(r0)),
            (Self::Integer(l0), Self::Integer(r0)) => Ok(l0.partial_cmp(r0)),
            (Self::Float(l0), Self::Float(r0)) => Ok(l0.partial_cmp(r0)),
            (Self::Agent(l0), Self::Agent(r0)) => Ok(l0.partial_cmp(r0)),
            (Self::ByteString(l0), Self::ByteString(r0)) => Ok(l0.partial_cmp(r0)),
            (Self::Integer(l0), Self::Float(r0)) => Ok((*l0 as f32).partial_cmp(r0)),
            (Self::Float(l0), Self::Integer(r0)) => Ok(l0.partial_cmp(&(*r0 as f32))),
            _ => Err(CaosError::new(ErrorType::TypeMismatch)),
        }
    }
}

impl Default for Variadic {
    fn default() -> Self {
        Self::Integer(0)
    }
}

impl From<String> for Variadic {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<i32> for Variadic {
    fn from(i: i32) -> Self {
        Self::Integer(i)
    }
}

impl From<f32> for Variadic {
    fn from(f: f32) -> Self {
        Self::Float(f)
    }
}

impl From<AgentRef> for Variadic {
    fn from(a: AgentRef) -> Self {
        Self::Agent(a)
    }
}

impl From<Vec<u8>> for Variadic {
    fn from(b: Vec<u8>) -> Self {
        Self::ByteString(b)
    }
}
