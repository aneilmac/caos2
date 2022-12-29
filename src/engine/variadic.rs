use crate::engine::AgentRef;

pub enum Variadic {
    String(String),
    Integer(i32),
    Float(f32),
    Agent(AgentRef),
    ByteString(Vec<u8>)
}

impl Variadic {
    pub fn is_string(&self) -> bool {
        match self {
            Self::String(..) => true,
            _ => false
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Self::Integer(..) => true,
            _ => false
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Self::Float(..) => true,
            _ => false
        }
    }

    pub fn is_agent(&self) -> bool {
        match self {
            Self::Agent(..) => true,
            _ => false
        }
    }

    pub fn is_byte_string(&self) -> bool {
        match self {
            Self::ByteString(..) => true,
            _ => false
        }
    }

    pub fn is_decimal(&self) -> bool {
        self.is_integer() || self.is_float()
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