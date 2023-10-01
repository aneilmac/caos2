use caos_macros::CaosParsable;
use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq, Clone, CaosParsable)]
pub enum ByteString {
    Raw(Vec<u8>),
}

impl From<Vec<u8>> for ByteString {
    fn from(v: Vec<u8>) -> Self {
        ByteString::Raw(v)
    }
}
