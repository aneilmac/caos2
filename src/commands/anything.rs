use crate::commands::{Agent, ByteString, Decimal, SString, Variable, Integer, Float};
use crate::parser::{CaosParsable, CaosParseResult};
use nom::branch::alt;
use nom::combinator::map;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Anything {
    Variable(Variable),
    String(SString),
    Decimal(Decimal),
    ByteString(ByteString),
    Agent(Agent),
}

impl From<Integer> for Anything {
    fn from(i: Integer) -> Self {
        Self::Decimal(i.into())
    }
}

impl From<Float> for Anything {
    fn from(f: Float) -> Self {
        Self::Decimal(f.into())
    }
}

impl From<Variable> for Anything {
    fn from(v: Variable) -> Self {
        Self::Variable(v)
    }
}

impl CaosParsable for Anything {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            // Important `Variable` is tested first,
            // as variable would be captured erroneously by
            // string/decimal/agent otherwise.
            map(Variable::parse_caos, Anything::Variable),
            map(SString::parse_caos, Anything::String),
            map(Decimal::parse_caos, Anything::Decimal),
            map(ByteString::parse_caos, Anything::ByteString),
            map(Agent::parse_caos, Anything::Agent),
        ))(input)
    }
}
