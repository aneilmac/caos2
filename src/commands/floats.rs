use super::{Agent, IntArg, FloatArg, SString};
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
pub enum Float {
    #[syntax(with_parser = "parse_literal")]
    Raw(LiteralF32),
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
        chemical: Box<IntArg>,
    },
    #[syntax]
    Dftx,
    #[syntax]
    Dfty,
    #[syntax]
    Driv {
        drive: Box<IntArg>,
    },
    #[syntax]
    Loci {
        r#type: Box<IntArg>,
        organ: Box<IntArg>,
        tissue: Box<IntArg>,
        id: Box<IntArg>,
    },
    #[syntax]
    Orgf {
        organ_number: Box<IntArg>,
        data: Box<IntArg>,
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
        room_id: Box<IntArg>,
        ca_index: Box<IntArg>,
    },
    #[syntax]
    Torx {
        room_id: Box<IntArg>,
    },
    #[syntax]
    Tory {
        room_id: Box<IntArg>,
    },
    #[syntax]
    Accg,
    #[syntax]
    Obst {
        direction: Box<IntArg>,
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
        x: Box<FloatArg>,
    },
    #[syntax]
    Asin {
        x: Box<FloatArg>,
    },
    #[syntax]
    Atan {
        x: Box<FloatArg>,
    },
    #[syntax(name = "cos_")]
    Cos {
        theta: Box<FloatArg>,
    },
    #[syntax]
    Itof {
        number_to_convert: Box<IntArg>,
    },
    #[syntax(name = "sin_")]
    Sin {
        theta: Box<FloatArg>,
    },
    #[syntax]
    Sqrt {
        value: Box<FloatArg>,
    },
    #[syntax]
    Stof {
        value: Box<SString>,
    },
    #[syntax(name = "tan_")]
    Tan {
        theta: Box<FloatArg>,
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

fn parse_literal(input: &str) -> CaosParseResult<&str, Float> {
    map(LiteralF32::parse_caos, Float::Raw)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_float() {
        let (_, res) = Float::parse_caos("3.134").expect("Valid float");
        assert_eq!(res, Float::Raw(LiteralF32(3.134)));
    }
}
