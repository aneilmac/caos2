use crate::{
    ast::{Agent, Anything, Decimal, DecimalArg, IntArg, Integer, SString, Variable},
    parser::CaosParser,
};
use pest::Parser;

use super::*;

#[test]
fn test_string_arg() {
    for p in CaosParser::parse(Rule::string_arg, r#""Hello""#).expect("Parsed") {
        assert_eq!(
            parse_string_arg(p).expect("Parsed variable"),
            SStringArg::String(SString::Literal("Hello".into())),
        );
    }
    for p in CaosParser::parse(Rule::float_arg, "MV32").expect("Parsed") {
        assert_eq!(
            parse_string_arg(p).expect("Parsed variable"),
            SStringArg::Variable(Variable::Mvxx(32)),
        );
    }
}

#[test]
fn test_string_literal() {
    for p in CaosParser::parse(Rule::string, r#""Hello""#).expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Literal("Hello".into()),
        );
    }
}

#[test]
fn test_string_catx() {
    for p in CaosParser::parse(Rule::string, "CATX ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Catx {
                category_id: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_hand() {
    for p in CaosParser::parse(Rule::string, "hand").expect("Parsed") {
        assert_eq!(parse_string(p).expect("Parsed variable"), SString::Hand,);
    }
}

#[test]
fn test_string_wild() {
    for p in CaosParser::parse(Rule::string, "wild ATTR aero Bhvr HAND byiT").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Wild {
                family: Box::new(IntArg::Primary(Integer::Attr)),
                genus: Box::new(IntArg::Primary(Integer::Aero)),
                species: Box::new(IntArg::Primary(Integer::Bhvr)),
                tag_stub: Box::new(SStringArg::String(SString::Hand)),
                offset: Box::new(IntArg::Primary(Integer::Byit)),
            },
        );
    }
}

#[test]
fn test_string_bkgd() {
    for p in CaosParser::parse(Rule::string, "bkgd ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Bkgd {
                metaroom_id: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_ptxt() {
    for p in CaosParser::parse(Rule::string, "ptxt").expect("Parsed") {
        assert_eq!(parse_string(p).expect("Parsed variable"), SString::Ptxt,);
    }
}

#[test]
fn test_string_face() {
    for p in CaosParser::parse(Rule::string, "face").expect("Parsed") {
        assert_eq!(parse_string(p).expect("Parsed variable"), SString::Face,);
    }
}

#[test]
fn test_string_dbg() {
    for p in CaosParser::parse(Rule::string, "dbg# ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Dbg {
                variable: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_dbga() {
    for p in CaosParser::parse(Rule::string, "dbga ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Dbga {
                variable: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_fvwm() {
    for p in CaosParser::parse(Rule::string, "fvwm face").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Fvwm {
                name: Box::new(SStringArg::String(SString::Face))
            },
        );
    }
}

#[test]
fn test_string_innl() {
    for p in CaosParser::parse(Rule::string, "innl").expect("Parsed") {
        assert_eq!(parse_string(p).expect("Parsed variable"), SString::Innl,);
    }
}

#[test]
fn test_string_gtos() {
    for p in CaosParser::parse(Rule::string, "gtos ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Gtos {
                slot: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_hist_foto() {
    for p in CaosParser::parse(Rule::string, "hist foto HAND ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::HistFoto {
                moniker: Box::new(SStringArg::String(SString::Hand)),
                event_no: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_hist_mon1() {
    for p in CaosParser::parse(Rule::string, "hist mon1 HAND ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::HistMon1 {
                moniker: Box::new(SStringArg::String(SString::Hand)),
                event_no: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_hist_mon2() {
    for p in CaosParser::parse(Rule::string, "hist mon2 HAND ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::HistMon2 {
                moniker: Box::new(SStringArg::String(SString::Hand)),
                event_no: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_hist_name() {
    for p in CaosParser::parse(Rule::string, "hist name HAND").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::HistName {
                moniker: Box::new(SStringArg::String(SString::Hand))
            },
        );
    }
}

#[test]
fn test_string_hist_next() {
    for p in CaosParser::parse(Rule::string, "hist next HAND").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::HistNext {
                moniker: Box::new(SStringArg::String(SString::Hand))
            },
        );
    }
}

#[test]
fn test_string_hist_prev() {
    for p in CaosParser::parse(Rule::string, "hist prev HAND").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::HistPrev {
                moniker: Box::new(SStringArg::String(SString::Hand))
            },
        );
    }
}

#[test]
fn test_string_hist_utxt() {
    for p in CaosParser::parse(Rule::string, "hist utxt HAND ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::HistUtxt {
                moniker: Box::new(SStringArg::String(SString::Hand)),
                event_no: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_hist_wnam() {
    for p in CaosParser::parse(Rule::string, "hist wnam HAND ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::HistWnam {
                moniker: Box::new(SStringArg::String(SString::Hand)),
                event_no: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_bkds() {
    for p in CaosParser::parse(Rule::string, "bkds ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Bkds {
                metaroom_id: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_emid() {
    for p in CaosParser::parse(Rule::string, "EMID").expect("Parsed") {
        assert_eq!(parse_string(p).expect("Parsed variable"), SString::Emid,);
    }
}

#[test]
fn test_string_erid() {
    for p in CaosParser::parse(Rule::string, "ERID ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Erid {
                metaroom_id: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_mloc() {
    for p in CaosParser::parse(Rule::string, "MLOC ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Mloc {
                metaroom_id: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_string_rate() {
    for p in CaosParser::parse(Rule::string, "RATE ATTR AERO").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Rate {
                room_type: Box::new(IntArg::Primary(Integer::Attr)),
                ca_index: Box::new(IntArg::Primary(Integer::Aero))
            }
        );
    }
}

#[test]
fn test_string_rloc() {
    for p in CaosParser::parse(Rule::string, "RLOc ATTR").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Rloc {
                room_id: Box::new(IntArg::Primary(Integer::Attr))
            }
        );
    }
}

#[test]
fn test_string_pray_agts() {
    for p in CaosParser::parse(Rule::string, "pray agts HAND EMID FACE").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::PrayAgts {
                resource_name: Box::new(SStringArg::String(SString::Hand)),
                string_tag: Box::new(SStringArg::String(SString::Emid)),
                default_value: Box::new(SStringArg::String(SString::Face))
            }
        );
    }
}

#[test]
fn test_string_pray_next() {
    for p in CaosParser::parse(Rule::string, "pray NEXT HAND Emid").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::PrayNext {
                resource_type: Box::new(SStringArg::String(SString::Hand)),
                last_known: Box::new(SStringArg::String(SString::Emid))
            }
        );
    }
}

#[test]
fn test_string_pray_prev() {
    for p in CaosParser::parse(Rule::string, "PRAY prev hand EMID").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::PrayPrev {
                resource_type: Box::new(SStringArg::String(SString::Hand)),
                last_known: Box::new(SStringArg::String(SString::Emid))
            }
        );
    }
}

#[test]
fn test_string_caos() {
    for p in CaosParser::parse(Rule::string, "CAOS ATTR AERO BASE CABB HAND CAGE attn MV00")
        .expect("Parsed")
    {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Caos {
                inline: Box::new(IntArg::Primary(Integer::Attr)),
                state_trans: Box::new(IntArg::Primary(Integer::Aero)),
                p1: Box::new(Anything::Decimal(Decimal::Integer(Integer::Base))),
                p2: Box::new(Anything::Decimal(Decimal::Integer(Integer::Cabb))),
                commands: Box::new(SStringArg::String(SString::Hand)),
                throws: Box::new(IntArg::Primary(Integer::Cage)),
                catches: Box::new(IntArg::Primary(Integer::Attn)),
                report: Box::new(Variable::Mvxx(0)),
            }
        );
    }
}

#[test]
fn test_string_rmsc() {
    for p in CaosParser::parse(Rule::string, "RMSC ATTR AERO").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Rmsc {
                x: Box::new(IntArg::Primary(Integer::Attr)),
                y: Box::new(IntArg::Primary(Integer::Aero)),
            }
        );
    }
}

#[test]
fn test_string_vois() {
    for p in CaosParser::parse(Rule::string, "VOIS").expect("Parsed") {
        assert_eq!(parse_string(p).expect("Parsed variable"), SString::Vois);
    }
}

#[test]
fn test_string_rtif() {
    for p in CaosParser::parse(Rule::string, "RTIF ATTR HAND").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Rtif {
                real_time: Box::new(IntArg::Primary(Integer::Attr)),
                format: Box::new(SStringArg::String(SString::Hand))
            }
        );
    }
}

#[test]
fn test_string_gamn() {
    for p in CaosParser::parse(Rule::string, "GAMN HAND").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Gamn {
                previous: Box::new(SStringArg::String(SString::Hand))
            }
        );
    }
}

#[test]
fn test_string_gnam() {
    for p in CaosParser::parse(Rule::string, "GNAM").expect("Parsed") {
        assert_eq!(parse_string(p).expect("Parsed variable"), SString::Gnam);
    }
}

#[test]
fn test_string_read() {
    for p in CaosParser::parse(Rule::string, "READ hAND ATTr").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Read {
                catalogue_tag: Box::new(SStringArg::String(SString::Hand)),
                offset: Box::new(IntArg::Primary(Integer::Attr))
            }
        );
    }
}

#[test]
fn test_string_subs() {
    for p in CaosParser::parse(Rule::string, "SUBS hAND ATTR AERO").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Subs {
                value: Box::new(SStringArg::String(SString::Hand)),
                start: Box::new(IntArg::Primary(Integer::Attr)),
                count: Box::new(IntArg::Primary(Integer::Aero))
            }
        );
    }
}

#[test]
fn test_string_vtos() {
    for p in CaosParser::parse(Rule::string, "VTOS %1").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Vtos {
                value: Box::new(DecimalArg::Decimal(Decimal::Integer(Integer::Literal(1))))
            }
        );
    }
}

#[test]
fn test_string_pswd() {
    for p in CaosParser::parse(Rule::string, "PSWD 0").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Pswd {
                world_index: Box::new(IntArg::Primary(Integer::Literal(0)))
            }
        );
    }
}

#[test]
fn test_string_wnam() {
    for p in CaosParser::parse(Rule::string, "WNAM").expect("Parsed") {
        assert_eq!(parse_string(p).expect("Parsed variable"), SString::Wnam);
    }
}

#[test]
fn test_string_wrld() {
    for p in CaosParser::parse(Rule::string, "wrld 0").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::Wrld {
                world_index: Box::new(IntArg::Primary(Integer::Literal(0)))
            }
        );
    }
}

#[test]
fn test_string_wuid() {
    for p in CaosParser::parse(Rule::string, "WUID").expect("Parsed") {
        assert_eq!(parse_string(p).expect("Parsed variable"), SString::Wuid);
    }
}

#[test]
fn test_prt_name() {
    for p in CaosParser::parse(Rule::string, "PRT: NAME NULL  0 1").expect("Parsed") {
        assert_eq!(
            parse_string(p).expect("Parsed variable"),
            SString::PrtName {
                agent: Box::new(Agent::Null.into()),
                in_or_out: Box::new(0.into()),
                port_index: Box::new(1.into())
            }
        );
    }
}
