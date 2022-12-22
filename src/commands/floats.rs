use super::{Agent, Integer, SString, Variable};
use crate::parser::CaosParsable;
use caos_macros::{CaosParsable, CommandList};
use nom::combinator::map;
use nom::number::complete::float;

/// We want strict equality for our tokens, so we wrap the f32 up and compare it bitwise to ensure
/// "true" equality for token parsing.
#[derive(Debug, Clone)]
pub struct F32Wrapper(f32);

impl PartialEq for F32Wrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for F32Wrapper {}

#[derive(CaosParsable, CommandList, Eq, PartialEq, Debug, Clone)]
pub enum Float {
    #[syntax(with_parser = "parse_literal")]
    Raw(F32Wrapper),
    #[syntax(with_parser = "parse_variable")]
    Variable(Box<Variable>),
    #[syntax]
    Disq { other: Box<Agent> },
    #[syntax]
    Fltx,
    #[syntax]
    Flty,
    #[syntax]
    Mthx,
    #[syntax]
    Mthy,
    #[syntax]
    Posb,
    #[syntax]
    Posl,
    #[syntax]
    Posr,
    #[syntax]
    Post,
    #[syntax]
    Posx,
    #[syntax]
    Posy,
    #[syntax]
    Rnge,
    #[syntax]
    Chem { chemical: Box<Integer> },
    #[syntax]
    Dftx,
    #[syntax]
    Dfty,
    #[syntax]
    Driv { drive: Box<Integer> },
    #[syntax]
    Loci {
        r#type: Box<Integer>,
        organ: Box<Integer>,
        tissue: Box<Integer>,
        id: Box<Integer>,
        new_value: Box<Float>,
    },
    #[syntax]
    Orgf {
        organ_number: Box<Integer>,
        data: Box<Integer>,
    },
    #[syntax]
    Uftx,
    #[syntax]
    Ufty,
    #[syntax]
    Innf,
    #[syntax]
    Movx,
    #[syntax]
    Movy,
    #[syntax]
    Prop {
        room_id: Box<Integer>,
        ca_index: Box<Integer>,
    },
    #[syntax]
    Torx { room_id: Box<Integer> },
    #[syntax]
    Tory { room_id: Box<Integer> },
    #[syntax]
    Accg,
    #[syntax]
    Obst { direction: Box<Integer> },
    #[syntax]
    Relx {
        first: Box<Agent>,
        second: Box<Agent>,
    },
    #[syntax]
    Rely {
        first: Box<Agent>,
        second: Box<Agent>,
    },
    #[syntax]
    Pace,
    #[syntax]
    Acos { x: Box<Float> },
    #[syntax]
    Asin { x: Box<Float> },
    #[syntax]
    Atan { x: Box<Float> },
    #[syntax(name = "cos_")]
    Cos { theta: Box<Float> },
    #[syntax]
    Itof { number_to_convert: Box<Integer> },
    #[syntax(name = "sin_")]
    Sin { theta: Box<Float> },
    #[syntax]
    Sqrt { value: Box<Float> },
    #[syntax]
    Stof { value: Box<SString> },
    #[syntax(name = "tan_")]
    Tan { theta: Box<Float> },
}

impl From<f32> for Float {
    fn from(f: f32) -> Self {
        Float::Raw(F32Wrapper(f))
    }
}

fn parse_literal(input: &str) -> nom::IResult<&str, Float> {
    map(float, |f| Float::Raw(F32Wrapper(f)))(input)
}

fn parse_variable(input: &str) -> nom::IResult<&str, Float> {
    map(Variable::parse_caos, |v| Float::Variable(Box::new(v)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_float() {
        let (_, res) = Float::parse_caos("3.134").expect("Valid float");
        assert_eq!(res, Float::Raw(F32Wrapper(3.134)));
    }

    #[test]
    fn test_variable_float() {
        let (_, res) = Float::parse_caos("velx").expect("Valid float");
        assert_eq!(res, Float::Variable(Box::new(Variable::Velx)));
    }
}
