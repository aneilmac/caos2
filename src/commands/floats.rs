use caos_macros::{CaosParsable, CommandList};

use super::{Agent, Integer, SString};

#[derive(CaosParsable, CommandList)]
pub enum Float {
    //#[raw]
    Raw(f32),
    //#[raw]
    Variable(String),
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
