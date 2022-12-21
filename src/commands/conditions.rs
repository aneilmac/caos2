use super::Anything;
use crate::parser::CaosParsable;

pub struct Condition {
    pub cond_type: ConditionType,
    pub lhs: Anything,
    pub rhs: Anything,
}

pub enum ConditionType {
    Eq,
    Ne,
    GE,
    GT,
    LE,
    LT,
}

impl CaosParsable for Condition {
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        todo!("TODO")
    }
}
