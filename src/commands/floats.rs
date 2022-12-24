use super::{Agent, Integer, SString, Variable};
use crate::parser::{CaosParsable, CaosParseResult};
use caos_macros::{CaosParsable, CommandList};
use nom::combinator::map;
use nom::number::complete::float;

/// We want strict equality for our tokens, so we wrap the f32 up and compare it bitwise to ensure
/// "true" equality for token parsing.
#[derive(Debug, Clone)]
pub struct LiteralF32(f32);

impl PartialEq for LiteralF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for LiteralF32 {}

impl From<f32> for LiteralF32 {
    fn from(f: f32) -> Self {
        LiteralF32(f)
    }
}

impl CaosParsable for LiteralF32 {
    fn parse_caos(input: &str) -> CaosParseResult<&str, Self>
    where
        Self: Sized,
    {
        map(float, LiteralF32)(input)
    }
}

#[derive(CaosParsable, CommandList, Eq, PartialEq, Debug, Clone)]
#[can_cast(Integer)]
pub enum Float {
    FromInteger(Box<Integer>),
    #[syntax(with_parser = "parse_literal")]
    Raw(LiteralF32),
    #[syntax(with_parser = "parse_variable")]
    Variable(Box<Variable>),
    #[syntax]
    Disq {
        other: Box<Agent>,
    },
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
    Chem {
        chemical: Box<Integer>,
    },
    #[syntax]
    Dftx,
    #[syntax]
    Dfty,
    #[syntax]
    Driv {
        drive: Box<Integer>,
    },
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
    Torx {
        room_id: Box<Integer>,
    },
    #[syntax]
    Tory {
        room_id: Box<Integer>,
    },
    #[syntax]
    Accg,
    #[syntax]
    Obst {
        direction: Box<Integer>,
    },
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
    Acos {
        x: Box<Float>,
    },
    #[syntax]
    Asin {
        x: Box<Float>,
    },
    #[syntax]
    Atan {
        x: Box<Float>,
    },
    #[syntax(name = "cos_")]
    Cos {
        theta: Box<Float>,
    },
    #[syntax]
    Itof {
        number_to_convert: Box<Integer>,
    },
    #[syntax(name = "sin_")]
    Sin {
        theta: Box<Float>,
    },
    #[syntax]
    Sqrt {
        value: Box<Float>,
    },
    #[syntax]
    Stof {
        value: Box<SString>,
    },
    #[syntax(name = "tan_")]
    Tan {
        theta: Box<Float>,
    },
}

impl From<f32> for Float {
    fn from(f: f32) -> Self {
        Float::Raw(f.into())
    }
}

impl From<LiteralF32> for Float {
    fn from(f: LiteralF32) -> Self {
        Float::Raw(f)
    }
}

impl From<Integer> for Float {
    fn from(i: Integer) -> Self {
        Float::FromInteger(Box::new(i))
    }
}

fn parse_literal(input: &str) -> CaosParseResult<&str, Float> {
    map(LiteralF32::parse_caos, Float::Raw)(input)
}

fn parse_variable(input: &str) -> CaosParseResult<&str, Float> {
    map(Variable::parse_caos, |v| Float::Variable(Box::new(v)))(input)
}

#[cfg(test)]
mod tests {
    use crate::commands::Command;

    use super::*;

    #[test]
    fn test_literal_float() {
        let (_, res) = Float::parse_caos("3.134").expect("Valid float");
        assert_eq!(res, Float::Raw(LiteralF32(3.134)));
    }

    #[test]
    fn test_variable_float() {
        let (_, res) = Float::parse_caos("velx").expect("Valid float");
        assert_eq!(res, Float::Variable(Box::new(Variable::Velx)));
    }

    #[test]
    fn test_integer_cast() {
        let (_, res) = Command::parse_caos("mvto rand 217 2787 1840").expect("Valid command");
        assert_eq!(
            res,
            Command::Mvto {
                y: 1840f32.into(),
                x: Float::FromInteger(Box::new(Integer::Rand {
                    value1: Box::new(217.into()),
                    value2: Box::new(2787.into())
                }))
            }
        );
    }
}
