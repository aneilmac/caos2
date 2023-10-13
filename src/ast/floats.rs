use super::{AgentArg, FloatArg, IntArg, SStringArg};

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

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Float {
    Literal(LitF32),
    Disq { other: Box<AgentArg> },
    Fltx,
    Flty,
    Mthx,
    Mthy,
    Posb,
    Posl,
    Posr,
    Post,
    Posx,
    Posy,
    Rnge,
    Chem { chemical: Box<IntArg> },
    Dftx,
    Dfty,
    Driv { drive: Box<IntArg> },
    Loci {
        r#type: Box<IntArg>,
        organ: Box<IntArg>,
        tissue: Box<IntArg>,
        id: Box<IntArg>,
    },
    Orgf {
        organ_number: Box<IntArg>,
        data: Box<IntArg>,
    },
    Uftx,
    Ufty,
    Innf,
    Movx,
    Movy,
    Prop {
        room_id: Box<IntArg>,
        ca_index: Box<IntArg>,
    },
    Torx { room_id: Box<IntArg> },
    Tory { room_id: Box<IntArg> },
    Accg,
    Obst { direction: Box<IntArg> },
    Relx {
        first: Box<AgentArg>,
        second: Box<AgentArg>,
    },
    Rely {
        first: Box<AgentArg>,
        second: Box<AgentArg>,
    },
    Pace,
    Acos { x: Box<FloatArg> },
    Asin { x: Box<FloatArg> },
    Atan { x: Box<FloatArg> },
    Cos { theta: Box<FloatArg> },
    Itof { number_to_convert: Box<IntArg> },
    Sin { theta: Box<FloatArg> },
    Sqrt { value: Box<FloatArg> },
    /// Converts a string in decimal to a floating point number.
    /// Characters in the string after an initial number are quietly ignored.
    /// If there is no obvious number then zero is returned.
    Stof { value: Box<SStringArg> },
    /// Returns tangent of theta. Theta should be in degrees.
    /// Watch out for those nasty discontinuities at 90 and 270.
    Tan { theta: Box<FloatArg> },
}

impl From<f32> for Float {
    fn from(f: f32) -> Self {
        Float::Literal(f.into())
    }
}
