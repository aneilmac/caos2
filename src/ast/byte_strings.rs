use caos_macros::CaosParsable;

#[derive(Debug, PartialEq, Eq, Clone, CaosParsable, Default)]
pub struct ByteString(Vec<u8>);

impl From<Vec<u8>> for ByteString {
    fn from(v: Vec<u8>) -> Self {
        ByteString(v)
    }
}
