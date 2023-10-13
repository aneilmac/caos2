

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ByteString(Vec<u8>);

impl From<Vec<u8>> for ByteString {
    fn from(v: Vec<u8>) -> Self {
        ByteString(v)
    }
}
