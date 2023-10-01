use caos_macros::CaosParsable;

#[derive(Eq, CaosParsable, PartialEq, Debug, Clone)]
pub struct Label(String);

impl From<String> for Label {
    fn from(s: String) -> Self {
        Label(s)
    }
}

impl From<Label> for String {
    fn from(l: Label) -> Self {
        l.0
    }
}
