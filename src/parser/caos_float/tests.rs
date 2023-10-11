use crate::{
    ast::{Agent, AgentArg, IntArg, Integer, SString, Variable},
    parser::CaosParser,
};
use pest::Parser;

use super::*;

#[test]
fn test_float_arg() {
    for p in CaosParser::parse(Rule::float_arg, "3.23").expect("Parsed") {
        assert_eq!(
            parse_float_arg(p).expect("Parsed variable"),
            FloatArg::Primary(Float::Literal(3.23.into())),
        );
    }
    for p in CaosParser::parse(Rule::float_arg, "3").expect("Parsed") {
        assert_eq!(
            parse_float_arg(p).expect("Parsed variable"),
            FloatArg::Castable(Integer::Literal(3)),
        );
    }
    for p in CaosParser::parse(Rule::float_arg, "MV32").expect("Parsed") {
        assert_eq!(
            parse_float_arg(p).expect("Parsed variable"),
            FloatArg::Variable(Variable::Mvxx(32)),
        );
    }
}

#[test]
fn test_float_literal() {
    for p in CaosParser::parse(Rule::float, "3.0").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Literal(3.0f32.into()),
        );
    }
}

#[test]
fn test_float_disq() {
    for p in CaosParser::parse(Rule::float, "DISQ PNTR").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Disq {
                other: Box::new(AgentArg::Agent(Agent::Pntr))
            },
        );
    }
}

#[test]
fn test_float_fltx() {
    for p in CaosParser::parse(Rule::float, "fltx").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Fltx,);
    }
}

#[test]
fn test_float_flty() {
    for p in CaosParser::parse(Rule::float, "flty").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Flty,);
    }
}

#[test]
fn test_float_mthx() {
    for p in CaosParser::parse(Rule::float, "mthx").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Mthx,);
    }
}

#[test]
fn test_float_mthy() {
    for p in CaosParser::parse(Rule::float, "MTHY").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Mthy,);
    }
}

#[test]
fn test_float_posb() {
    for p in CaosParser::parse(Rule::float, "PoSB").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Posb,);
    }
}

#[test]
fn test_float_posl() {
    for p in CaosParser::parse(Rule::float, "PoSl").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Posl,);
    }
}

#[test]
fn test_float_posr() {
    for p in CaosParser::parse(Rule::float, "PoSr").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Posr,);
    }
}

#[test]
fn test_float_post() {
    for p in CaosParser::parse(Rule::float, "POST").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Post,);
    }
}

#[test]
fn test_float_posx() {
    for p in CaosParser::parse(Rule::float, "POSX").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Posx,);
    }
}

#[test]
fn test_float_posy() {
    for p in CaosParser::parse(Rule::float, "POSY").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Posy,);
    }
}

#[test]
fn test_float_rnge() {
    for p in CaosParser::parse(Rule::float, "RNGE").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Rnge,);
    }
}

#[test]
fn test_float_chem() {
    for p in CaosParser::parse(Rule::float, "CHEM 0").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Chem {
                chemical: Box::new(IntArg::Primary(Integer::Literal(0)))
            },
        );
    }
}

#[test]
fn test_float_dftx() {
    for p in CaosParser::parse(Rule::float, "DFTX").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Dftx,);
    }
}

#[test]
fn test_float_dfty() {
    for p in CaosParser::parse(Rule::float, "DFTY").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Dfty,);
    }
}

#[test]
fn test_float_driv() {
    for p in CaosParser::parse(Rule::float, "DRIV 1").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Driv {
                drive: Box::new(IntArg::Primary(Integer::Literal(1)))
            },
        );
    }
}

#[test]
fn test_float_loci() {
    for p in CaosParser::parse(Rule::float, "LOCI 1 2 3 4").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Loci {
                r#type: Box::new(IntArg::Primary(Integer::Literal(1))),
                organ: Box::new(IntArg::Primary(Integer::Literal(2))),
                tissue: Box::new(IntArg::Primary(Integer::Literal(3))),
                id: Box::new(IntArg::Primary(Integer::Literal(4))),
            },
        );
    }
}

#[test]
fn test_float_orgf() {
    for p in CaosParser::parse(Rule::float, "ORGF 1 2").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Orgf {
                organ_number: Box::new(IntArg::Primary(Integer::Literal(1))),
                data: Box::new(IntArg::Primary(Integer::Literal(2)))
            },
        );
    }
}

#[test]
fn test_float_uftx() {
    for p in CaosParser::parse(Rule::float, "UFTX").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Uftx,);
    }
}

#[test]
fn test_float_ufty() {
    for p in CaosParser::parse(Rule::float, "UFTY").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Ufty,);
    }
}

#[test]
fn test_float_innf() {
    for p in CaosParser::parse(Rule::float, "innf").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Innf,);
    }
}

#[test]
fn test_float_movx() {
    for p in CaosParser::parse(Rule::float, "MOVX").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Movx,);
    }
}

#[test]
fn test_float_movy() {
    for p in CaosParser::parse(Rule::float, "MOVY").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Movy,);
    }
}

#[test]
fn test_float_prop() {
    for p in CaosParser::parse(Rule::float, "PROP 1 2").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Prop {
                room_id: Box::new(IntArg::Primary(Integer::Literal(1))),
                ca_index: Box::new(IntArg::Primary(Integer::Literal(2)))
            },
        );
    }
}

#[test]
fn test_float_torx() {
    for p in CaosParser::parse(Rule::float, "TORX 1").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Torx {
                room_id: Box::new(IntArg::Primary(Integer::Literal(1)))
            },
        );
    }
}

#[test]
fn test_float_tory() {
    for p in CaosParser::parse(Rule::float, "TORY 1").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Tory {
                room_id: Box::new(IntArg::Primary(Integer::Literal(1)))
            },
        );
    }
}

#[test]
fn test_float_accg() {
    for p in CaosParser::parse(Rule::float, "ACCG").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Accg,);
    }
}

#[test]
fn test_float_obst() {
    for p in CaosParser::parse(Rule::float, "OBST 1").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Obst {
                direction: Box::new(IntArg::Primary(Integer::Literal(1)))
            },
        );
    }
}

#[test]
fn test_float_relx() {
    for p in CaosParser::parse(Rule::float, "RELX PNTR _IT_").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Relx {
                first: Box::new(AgentArg::Agent(Agent::Pntr)),
                second: Box::new(AgentArg::Agent(Agent::It))
            },
        );
    }
}

#[test]
fn test_float_rely() {
    for p in CaosParser::parse(Rule::float, "RELY PNTR _IT_").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Rely {
                first: Box::new(AgentArg::Agent(Agent::Pntr)),
                second: Box::new(AgentArg::Agent(Agent::It))
            },
        );
    }
}

#[test]
fn test_float_pace() {
    for p in CaosParser::parse(Rule::float, "PACE").expect("Parsed") {
        assert_eq!(parse_float(p).expect("Parsed variable"), Float::Pace,);
    }
}

#[test]
fn test_float_acos() {
    for p in CaosParser::parse(Rule::float, "acos 1").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Acos {
                x: Box::new(FloatArg::Castable(Integer::Literal(1)))
            },
        );
    }
}

#[test]
fn test_float_asin() {
    for p in CaosParser::parse(Rule::float, "asin 1.0").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Asin {
                x: Box::new(FloatArg::Primary(Float::Literal(1.0f32.into())))
            },
        );
    }
}

#[test]
fn test_float_atan() {
    for p in CaosParser::parse(Rule::float, "atan PACE").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Atan {
                x: Box::new(FloatArg::Primary(Float::Pace))
            },
        );
    }
}

#[test]
fn test_float_cos() {
    for p in CaosParser::parse(Rule::float, "cos_ accg").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Cos {
                theta: Box::new(FloatArg::Primary(Float::Accg))
            },
        );
    }
}

#[test]
fn test_float_itof() {
    for p in CaosParser::parse(Rule::float, "itof 3").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Itof {
                number_to_convert: Box::new(IntArg::Primary(Integer::Literal(3)))
            },
        );
    }
}

#[test]
fn test_float_sin() {
    for p in CaosParser::parse(Rule::float, "sin_ MOVX").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Sin {
                theta: Box::new(FloatArg::Primary(Float::Movx))
            },
        );
    }
}

#[test]
fn test_float_sqrt() {
    for p in CaosParser::parse(Rule::float, "sqrt sin_ MOVX").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Sqrt {
                value: Box::new(FloatArg::Primary(Float::Sin {
                    theta: Box::new(FloatArg::Primary(Float::Movx))
                }))
            },
        );
    }
}

#[test]
fn test_float_stof() {
    for p in CaosParser::parse(Rule::float, r#"stof "3.0""#).expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Stof {
                value: Box::new(crate::ast::SStringArg::String(SString::Literal(
                    "3.0".to_owned()
                )))
            },
        );
    }
}

#[test]
fn test_float_tan() {
    for p in CaosParser::parse(Rule::float, "tan_ MOVX").expect("Parsed") {
        assert_eq!(
            parse_float(p).expect("Parsed variable"),
            Float::Tan {
                theta: Box::new(FloatArg::Primary(Float::Movx))
            },
        );
    }
}
