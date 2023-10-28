use std::{convert::Infallible, error::Error};

pub type Result<T> = std::result::Result<T, CaosError>;

type LineCol = (usize, usize);

#[derive(Debug)]
pub enum ErrorType {
    ParseError {
        line_col: LineCol,
    },
    CastError {
        line_col: Option<LineCol>,
    },
    ArgErr {
        expected: usize,
        actual: usize,
        line_col: LineCol,
    },
    EndOfStream,
    SubError(Box<dyn Error>),
}

#[derive(Debug)]
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

    pub fn new_end_of_stream() -> Self {
        CaosError::new(
            ErrorType::EndOfStream,
            String::from("Stream ended unexpectedly"),
        )
    }

    pub fn new_from_error(e: Box<dyn Error>) -> Self {
        CaosError::new(ErrorType::SubError(e), String::new())
    }

    pub fn new_arg_count_error(expected: usize, actual: usize, line_col: LineCol) -> Self {
        CaosError::new(
            ErrorType::ArgErr {
                expected,
                actual,
                line_col,
            },
            format!(
                "Arg count error at line {} col: {}. Expected: {}, actual: {}.",
                line_col.0, line_col.1, expected, actual
            ),
        )
    }

    pub(crate) fn new_parse_error(p: pest::iterators::Pair<crate::Rule>) -> Self {
        let line_col @ (line, col) = p.line_col();
        let error_type = ErrorType::ParseError { line_col };
        let message = format!(
            "Parse error at line: {} col: {}, reading {}.",
            line,
            col,
            p.as_str()
        );
        CaosError {
            error_type,
            message,
        }
    }
}

impl Error for CaosError {}

impl From<Infallible> for CaosError {
    fn from(x: Infallible) -> Self {
        match x {}
    }
}

impl std::fmt::Display for CaosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}
