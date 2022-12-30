use crate::commands::{Agent, ByteString, Decimal, Float, Integer, SString, Variable};
use crate::engine::{EvaluateCommand, ScriptRefMut};
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

impl EvaluateCommand for Anything {
    type ReturnType = crate::engine::Variadic;
    fn evaluate(&self, script: &mut ScriptRefMut<'_>) -> crate::Result<Self::ReturnType> {
        use crate::engine::Variadic;
        match self {
            Anything::Variable(v) => v.evaluate(script),
            Anything::String(s) => s.evaluate(script).map(Variadic::from),
            Anything::Decimal(d) => d.evaluate(script).map(Variadic::from),
            Anything::ByteString(b) => b.evaluate(script).map(Variadic::from),
            Anything::Agent(a) => a.evaluate(script).map(Variadic::from),
        }
    }
}
