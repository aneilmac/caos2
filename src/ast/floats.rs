use super::{AgentArg, FloatArg, IntArg, SStringArg};
use crate::Rule;
use caos_macros::ExpressionParser;

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

#[derive(Eq, PartialEq, Debug, Clone, ExpressionParser)]
pub enum Float {
    #[parse(ignore)]
    Literal(LitF32),
    #[parse(rule=Rule::float_disq)]
    Disq { other: Box<AgentArg> },
    #[parse(rule=Rule::float_fltx)]
    Fltx,
    #[parse(rule=Rule::float_flty)]
    Flty,
    #[parse(rule=Rule::float_mthx)]
    Mthx,
    #[parse(rule=Rule::float_mthy)]
    Mthy,
    #[parse(rule=Rule::float_posb)]
    Posb,
    #[parse(rule=Rule::float_posl)]
    Posl,
    #[parse(rule=Rule::float_posr)]
    Posr,
    #[parse(rule=Rule::float_post)]
    Post,
    #[parse(rule=Rule::float_posx)]
    Posx,
    #[parse(rule=Rule::float_posy)]
    Posy,
    #[parse(rule=Rule::overloaded_rnge)]
    Rnge,
    #[parse(rule=Rule::overloaded_chem)]
    Chem { chemical: Box<IntArg> },
    #[parse(rule=Rule::float_dftx)]
    Dftx,
    #[parse(rule=Rule::float_dfty)]
    Dfty,
    #[parse(rule=Rule::overloaded_driv)]
    Driv { drive: Box<IntArg> },
    #[parse(rule=Rule::overloaded_loci)]
    Loci {
        r#type: Box<IntArg>,
        organ: Box<IntArg>,
        tissue: Box<IntArg>,
        id: Box<IntArg>,
    },
    #[parse(rule=Rule::float_orgf)]
    Orgf {
        organ_number: Box<IntArg>,
        data: Box<IntArg>,
    },
    #[parse(rule=Rule::float_uftx)]
    Uftx,
    #[parse(rule=Rule::float_ufty)]
    Ufty,
    #[parse(rule=Rule::float_innf)]
    Innf,
    #[parse(rule=Rule::float_movx)]
    Movx,
    #[parse(rule=Rule::float_movy)]
    Movy,
    #[parse(rule=Rule::overloaded_prop)]
    Prop {
        room_id: Box<IntArg>,
        ca_index: Box<IntArg>,
    },
    #[parse(rule=Rule::float_torx)]
    Torx { room_id: Box<IntArg> },
    #[parse(rule=Rule::float_tory)]
    Tory { room_id: Box<IntArg> },
    #[parse(rule=Rule::overloaded_accg)]
    Accg,
    #[parse(rule=Rule::float_obst)]
    Obst { direction: Box<IntArg> },
    #[parse(rule=Rule::float_relx)]
    Relx {
        first: Box<AgentArg>,
        second: Box<AgentArg>,
    },
    #[parse(rule=Rule::float_rely)]
    Rely {
        first: Box<AgentArg>,
        second: Box<AgentArg>,
    },
    #[parse(rule=Rule::float_pace)]
    Pace,
    #[parse(rule=Rule::float_acos)]
    Acos { x: Box<FloatArg> },
    #[parse(rule=Rule::float_asin)]
    Asin { x: Box<FloatArg> },
    #[parse(rule=Rule::float_atan)]
    Atan { x: Box<FloatArg> },
    #[parse(rule=Rule::float_cos)]
    Cos { theta: Box<FloatArg> },
    #[parse(rule=Rule::float_itof)]
    Itof { number_to_convert: Box<IntArg> },
    #[parse(rule=Rule::float_sin)]
    Sin { theta: Box<FloatArg> },
    #[parse(rule=Rule::float_sqrt)]
    Sqrt { value: Box<FloatArg> },
    /// Converts a string in decimal to a floating point number.
    /// Characters in the string after an initial number are quietly ignored.
    /// If there is no obvious number then zero is returned.
    #[parse(rule=Rule::float_stof)]
    Stof { value: Box<SStringArg> },
    /// Returns tangent of theta. Theta should be in degrees.
    /// Watch out for those nasty discontinuities at 90 and 270.
    #[parse(rule=Rule::float_tan)]
    Tan { theta: Box<FloatArg> },
}

impl From<f32> for Float {
    fn from(f: f32) -> Self {
        Float::Literal(f.into())
    }
}
