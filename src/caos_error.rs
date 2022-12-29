use std::error::Error;

pub type Result<T> = std::result::Result<T, CaosError>;

#[derive(Debug, Clone)]
pub enum ErrorType {
    BlownStack,
    DecimalConversionFailure,
    BadRegister,
    ParseError,
}

#[derive(Debug, Clone)]
pub struct CaosError {
    pub error_type: ErrorType,
}

impl CaosError {
    pub fn new(error_type: ErrorType) -> Self {
        Self { error_type }
    }
}

impl Error for CaosError {}

impl std::fmt::Display for CaosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.error_type)
    }
}
