use crate::parser::CaosParsable;

pub struct Label(String);

impl CaosParsable for Label {
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        todo!("TODO")
    }
}
