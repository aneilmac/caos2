use super::{Agent, FloatArg, IntArg, SString};
use caos_macros::{CaosParsable, CommandList};

#[derive(PartialEq, Debug, Clone)]
pub struct LitF32(f32);

impl Eq for LitF32 {}

impl From<LitF32> for f32 {
    fn from(l: LitF32) -> f32 {
        l.0
    }
}

impl From<f32> for LitF32 {
    fn from(l: f32) -> LitF32 {
        LitF32(l)
    }
}

#[derive(CaosParsable, CommandList, Eq, PartialEq, Debug, Clone)]
pub enum Float {
    #[syntax(with_parser = "parse_literal")]
    Literal(LitF32),
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
    Chem { chemical: Box<IntArg> },
    #[syntax]
    Dftx,
    #[syntax]
    Dfty,
    #[syntax]
    Driv { drive: Box<IntArg> },
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
    Torx { room_id: Box<IntArg> },
    #[syntax]
    Tory { room_id: Box<IntArg> },
    #[syntax]
    Accg,
    #[syntax]
    Obst { direction: Box<IntArg> },
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
    Acos { x: Box<FloatArg> },
    #[syntax]
    Asin { x: Box<FloatArg> },
    #[syntax]
    Atan { x: Box<FloatArg> },
    #[syntax(name = "cos_")]
    Cos { theta: Box<FloatArg> },
    #[syntax]
    Itof { number_to_convert: Box<IntArg> },
    #[syntax(name = "sin_")]
    Sin { theta: Box<FloatArg> },
    #[syntax]
    Sqrt { value: Box<FloatArg> },
    /// Converts a string in decimal to a floating point number.
    /// Characters in the string after an initial number are quietly ignored.
    /// If there is no obvious number then zero is returned.
    #[syntax]
    Stof { value: Box<SString> },
    /// Returns tangent of theta. Theta should be in degrees.
    /// Watch out for those nasty discontinuities at 90 and 270.
    #[syntax(name = "tan_")]
    Tan { theta: Box<FloatArg> },
}

impl From<f32> for Float {
    fn from(f: f32) -> Self {
        Float::Literal(f.into())
    }
}
