use std::error::Error;

pub type Result<T> = std::result::Result<T, CaosError>;

#[derive(Debug, Clone)]
pub enum ErrorType {
    BlownStack,
    DecimalConversionFailure,
    BadRegister,
    ParseError { line: usize, col: usize },
    TypeMismatch,
    TooManyInstallScripts,
    TooManyRemovalScripts,
}

#[derive(Debug, Clone)]
pub struct CaosError {
    pub error_type: ErrorType,
    message: String,
}

impl CaosError {
    pub fn new(error_type: ErrorType, message: String) -> Self {
        Self {
            error_type,
            message,
        }
    }

    pub(crate) fn new_parse_error(p: pest::iterators::Pair<crate::Rule>) -> Self {
        let (line, col) = p.line_col();
        let error_type = ErrorType::ParseError { line, col };
        let message = format!(
            "Parse error at line: {} col: {}, reading {}. Got `{:?}` token.",
            line,
            col,
            p.as_str(),
            p.as_rule()
        );
        CaosError {
            error_type,
            message,
        }
    }
}

impl Error for CaosError {}

impl std::fmt::Display for CaosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}
