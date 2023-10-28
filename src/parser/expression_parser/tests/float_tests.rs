use super::parse_expr;
use crate::ast::{Agent, Float, FloatArg, Variable};

fn parse_float_arg(content: &str) -> FloatArg {
    parse_expr::<FloatArg>(content)
}

fn parse_float(content: &str) -> Float {
    let a = parse_float_arg(content);
    match a {
        FloatArg::Primary(f) => f,
        _ => panic!("Expected Float"),
    }
}

#[test]
fn test_float_arg() {
    let p = parse_float_arg("3.23");
    assert_eq!(p, 3.23f32.into());

    let p = parse_float_arg("3");
    assert_eq!(p, 3i32.into());

    let p = parse_float_arg("MV32");
    assert_eq!(p, Variable::Mvxx(32).into());
}

#[test]
fn test_float_literal() {
    let p = parse_float("3.0");
    assert_eq!(p, 3.0f32.into());
}

#[test]
fn test_float_disq() {
    let p = parse_float("DISQ PNTR");
    assert_eq!(
        p,
        Float::Disq {
            other: Box::new(Agent::Pntr.into())
        }
    );
}

#[test]
fn test_float_fltx() {
    let p = parse_float("FLTX");
    assert_eq!(p, Float::Fltx);
}

#[test]
fn test_float_flty() {
    let p = parse_float("FLTY");
    assert_eq!(p, Float::Flty);
}

#[test]
fn test_float_mthx() {
    let p = parse_float("MTHX");
    assert_eq!(p, Float::Mthx);
}

#[test]
fn test_float_mthy() {
    let p = parse_float("MTHY");
    assert_eq!(p, Float::Mthy);
}

#[test]
fn test_float_posb() {
    let p = parse_float("POSB");
    assert_eq!(p, Float::Posb);
}

#[test]
fn test_float_posl() {
    let p = parse_float("POSL");
    assert_eq!(p, Float::Posl);
}

#[test]
fn test_float_posr() {
    let p = parse_float("POSR");
    assert_eq!(p, Float::Posr);
}

#[test]
fn test_float_post() {
    let p = parse_float("POST");
    assert_eq!(p, Float::Post);
}

#[test]
fn test_float_posx() {
    let p = parse_float("POSX");
    assert_eq!(p, Float::Posx);
}

#[test]
fn test_float_posy() {
    let p = parse_float("POSY");
    assert_eq!(p, Float::Posy);
}

#[test]
fn test_float_rnge() {
    let p = parse_float("RNGE");
    assert_eq!(p, Float::Rnge);
}

#[test]
fn test_float_chem() {
    let p = parse_float("CHEM 0");
    assert_eq!(
        p,
        Float::Chem {
            chemical: Box::new(0.into())
        }
    );
}

#[test]
fn test_float_dftx() {
    let p = parse_float("DFTX");
    assert_eq!(p, Float::Dftx);
}

#[test]
fn test_float_dfty() {
    let p = parse_float("DFTY");
    assert_eq!(p, Float::Dfty);
}

#[test]
fn test_float_driv() {
    let p = parse_float("DRIV 1");
    assert_eq!(
        p,
        Float::Driv {
            drive: Box::new(1.into())
        }
    );
}

#[test]
fn test_float_loci() {
    let p = parse_float("LOCI 1 2 3 4");
    assert_eq!(
        p,
        Float::Loci {
            r#type: Box::new(1.into()),
            organ: Box::new(2.into()),
            tissue: Box::new(3.into()),
            id: Box::new(4.into()),
        }
    );
}

#[test]
fn test_float_orgf() {
    let p = parse_float("ORGF 1 2");
    assert_eq!(
        p,
        Float::Orgf {
            organ_number: Box::new(1.into()),
            data: Box::new(2.into())
        },
    );
}

#[test]
fn test_float_uftx() {
    let p = parse_float("UFTX");
    assert_eq!(p, Float::Uftx);
}

#[test]
fn test_float_ufty() {
    let p = parse_float("UFTY");
    assert_eq!(p, Float::Ufty);
}

#[test]
fn test_float_innf() {
    let p = parse_float("INNF");
    assert_eq!(p, Float::Innf);
}

#[test]
fn test_float_movx() {
    let p = parse_float("MOVX");
    assert_eq!(p, Float::Movx);
}

#[test]
fn test_float_movy() {
    let p = parse_float("MOVY");
    assert_eq!(p, Float::Movy);
}

#[test]
fn test_float_prop() {
    let p = parse_float("PROP 1 2");
    assert_eq!(
        p,
        Float::Prop {
            room_id: Box::new(1.into()),
            ca_index: Box::new(2.into())
        }
    );
}

#[test]
fn test_float_torx() {
    let p = parse_float("TORX 1");
    assert_eq!(
        p,
        Float::Torx {
            room_id: Box::new(1.into())
        },
    );
}

#[test]
fn test_float_tory() {
    let p = parse_float("TORY 1");
    assert_eq!(
        p,
        Float::Tory {
            room_id: Box::new(1.into())
        },
    );
}

#[test]
fn test_float_accg() {
    let p = parse_float("ACCG");
    assert_eq!(p, Float::Accg);
}

#[test]
fn test_float_obst() {
    let p = parse_float("OBST 1");
    assert_eq!(
        p,
        Float::Obst {
            direction: Box::new(1.into())
        }
    );
}

#[test]
fn test_float_relx() {
    let p = parse_float("RELX PNTR _IT_");
    assert_eq!(
        p,
        Float::Relx {
            first: Box::new(Agent::Pntr.into()),
            second: Box::new(Agent::It.into())
        }
    );
}

#[test]
fn test_float_rely() {
    let p = parse_float("RELY PNTR _IT_");
    assert_eq!(
        p,
        Float::Rely {
            first: Box::new(Agent::Pntr.into()),
            second: Box::new(Agent::It.into())
        }
    );
}

#[test]
fn test_float_pace() {
    let p = parse_float("PACE");
    assert_eq!(p, Float::Pace);
}

#[test]
fn test_float_acos() {
    let p = parse_float("ACOS 1");
    assert_eq!(
        p,
        Float::Acos {
            x: Box::new(1i32.into())
        }
    );
}

#[test]
fn test_float_asin() {
    let p = parse_float("ASIN 1.0");
    assert_eq!(
        p,
        Float::Asin {
            x: Box::new(1.0f32.into())
        }
    );
}

#[test]
fn test_float_atan() {
    let p = parse_float("ATAN PACE");
    assert_eq!(
        p,
        Float::Atan {
            x: Box::new(Float::Pace.into())
        }
    );
}

#[test]
fn test_float_cos() {
    let p = parse_float("COS_ ACCG");
    assert_eq!(
        p,
        Float::Cos {
            theta: Box::new(Float::Accg.into())
        }
    );
}

#[test]
fn test_float_itof() {
    let p = parse_float("ITOF 3");
    assert_eq!(
        p,
        Float::Itof {
            number_to_convert: Box::new(3.into())
        }
    );
}

#[test]
fn test_float_sin() {
    let p = parse_float("SIN_ MOVX");
    assert_eq!(
        p,
        Float::Sin {
            theta: Box::new(Float::Movx.into())
        }
    );
}

#[test]
fn test_float_sqrt() {
    let p = parse_float("SQRT SIN_ MOVX");
    assert_eq!(
        p,
        Float::Sqrt {
            value: Box::new(
                Float::Sin {
                    theta: Box::new(Float::Movx.into())
                }
                .into()
            )
        }
    );
}

#[test]
fn test_float_stof() {
    let p = parse_float(r#"STOF "3.0""#);
    assert_eq!(
        p,
        Float::Stof {
            value: Box::new(String::from("3.0").into())
        }
    );
}

#[test]
fn test_float_tan() {
    let p = parse_float("TAN_ MOVX");
    assert_eq!(
        p,
        Float::Tan {
            theta: Box::new(Float::Movx.into())
        }
    );
}
