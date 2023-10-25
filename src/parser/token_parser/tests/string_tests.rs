use super::parse_expr;
use crate::ast::{Agent, Integer, SString, SStringArg, Variable};

fn parse_sstring_arg(content: &str) -> SStringArg {
    parse_expr::<SStringArg>(content)
}

fn parse_sstring(content: &str) -> SString {
    let a = parse_sstring_arg(content);
    match a {
        SStringArg::String(s) => s,
        _ => panic!("Expected String"),
    }
}

#[test]
fn test_string_arg() {
    let p = parse_sstring_arg(r#""Hello""#);
    assert_eq!(p, String::from("Hello").into());

    let p = parse_sstring_arg("MV32");
    assert_eq!(p, Variable::Mvxx(32).into());
}

#[test]
fn test_string_literal() {
    let p = parse_sstring(r#""Hello""#);
    assert_eq!(p, String::from("Hello").into());
}

#[test]
fn test_string_catx() {
    let p = parse_sstring("CATX ATTR");
    assert_eq!(
        p,
        SString::Catx {
            category_id: Box::new(Integer::Attr.into())
        },
    );
}

#[test]
fn test_string_hand() {
    let p = parse_sstring("HAND");
    assert_eq!(p, SString::Hand);
}

#[test]
fn test_string_wild() {
    let p = parse_sstring("WILD ATTR AERO BHVR HAND BYIT");
    assert_eq!(
        p,
        SString::Wild {
            family: Box::new(Integer::Attr.into()),
            genus: Box::new(Integer::Aero.into()),
            species: Box::new(Integer::Bhvr.into()),
            tag_stub: Box::new(SString::Hand.into()),
            offset: Box::new(Integer::Byit.into()),
        }
    );
}

#[test]
fn test_string_bkgd() {
    let p = parse_sstring("BKGD ATTR");
    assert_eq!(
        p,
        SString::Bkgd {
            metaroom_id: Box::new(Integer::Attr.into())
        },
    );
}

#[test]
fn test_string_ptxt() {
    let p = parse_sstring("PTXT");
    assert_eq!(p, SString::Ptxt);
}

#[test]
fn test_string_face() {
    let p = parse_sstring("FACE");
    assert_eq!(p, SString::Face);
}

#[test]
fn test_string_dbg() {
    let p = parse_sstring("DBG# ATTR");
    assert_eq!(
        p,
        SString::Dbg {
            variable: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_dbga() {
    let p = parse_sstring("DBGA ATTR");
    assert_eq!(
        p,
        SString::Dbga {
            variable: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_fvwm() {
    let p = parse_sstring("FVWM FACE");
    assert_eq!(
        p,
        SString::Fvwm {
            name: Box::new(SString::Face.into())
        }
    );
}

#[test]
fn test_string_innl() {
    let p = parse_sstring("INNL");
    assert_eq!(p, SString::Innl);
}

#[test]
fn test_string_gtos() {
    let p = parse_sstring("GTOS ATTR");
    assert_eq!(
        p,
        SString::Gtos {
            slot: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_hist_foto() {
    let p = parse_sstring("HIST FOTO HAND ATTR");
    assert_eq!(
        p,
        SString::HistFoto {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_hist_mon1() {
    let p = parse_sstring("HIST MON1 HAND ATTR");
    assert_eq!(
        p,
        SString::HistMon1 {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_hist_mon2() {
    let p = parse_sstring("HIST MON2 HAND ATTR");
    assert_eq!(
        p,
        SString::HistMon2 {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_hist_name() {
    let p = parse_sstring("HIST NAME HAND");
    assert_eq!(
        p,
        SString::HistName {
            moniker: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_string_hist_next() {
    let p = parse_sstring("HIST NEXT HAND");
    assert_eq!(
        p,
        SString::HistNext {
            moniker: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_string_hist_prev() {
    let p = parse_sstring("HIST PREV HAND");
    assert_eq!(
        p,
        SString::HistPrev {
            moniker: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_string_hist_utxt() {
    let p = parse_sstring("HIST UTXT HAND ATTR");
    assert_eq!(
        p,
        SString::HistUtxt {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_hist_wnam() {
    let p = parse_sstring("HIST WNAM HAND ATTR");
    assert_eq!(
        p,
        SString::HistWnam {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_bkds() {
    let p = parse_sstring("BKDS ATTR");
    assert_eq!(
        p,
        SString::Bkds {
            metaroom_id: Box::new(Integer::Attr.into())
        },
    );
}

#[test]
fn test_string_emid() {
    let p = parse_sstring("EMID");
    assert_eq!(p, SString::Emid);
}

#[test]
fn test_string_erid() {
    let p = parse_sstring("ERID ATTR");
    assert_eq!(
        p,
        SString::Erid {
            metaroom_id: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_mloc() {
    let p = parse_sstring("MLOC ATTR");
    assert_eq!(
        p,
        SString::Mloc {
            metaroom_id: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_rate() {
    let p = parse_sstring("RATE ATTR AERO");
    assert_eq!(
        p,
        SString::Rate {
            room_type: Box::new(Integer::Attr.into()),
            ca_index: Box::new(Integer::Aero.into())
        }
    );
}

#[test]
fn test_string_rloc() {
    let p = parse_sstring("RLOC ATTR");
    assert_eq!(
        p,
        SString::Rloc {
            room_id: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_pray_agts() {
    let p = parse_sstring("PRAY AGTS HAND EMID FACE");
    assert_eq!(
        p,
        SString::PrayAgts {
            resource_name: Box::new(SString::Hand.into()),
            string_tag: Box::new(SString::Emid.into()),
            default_value: Box::new(SString::Face.into())
        }
    );
}

#[test]
fn test_string_pray_next() {
    let p = parse_sstring("PRAY NEXT HAND EMID");
    assert_eq!(
        p,
        SString::PrayNext {
            resource_type: Box::new(SString::Hand.into()),
            last_known: Box::new(SString::Emid.into())
        }
    );
}

#[test]
fn test_string_pray_prev() {
    let p = parse_sstring("PRAY PREV HAND EMID");
    assert_eq!(
        p,
        SString::PrayPrev {
            resource_type: Box::new(SString::Hand.into()),
            last_known: Box::new(SString::Emid.into())
        }
    );
}

#[test]
fn test_string_caos() {
    let p = parse_sstring("CAOS ATTR AERO BASE CABB HAND CAGE ATTN MV00");
    assert_eq!(
        p,
        SString::Caos {
            inline: Box::new(Integer::Attr.into()),
            state_trans: Box::new(Integer::Aero.into()),
            p1: Box::new(Integer::Base.into()),
            p2: Box::new(Integer::Cabb.into()),
            commands: Box::new(SString::Hand.into()),
            throws: Box::new(Integer::Cage.into()),
            catches: Box::new(Integer::Attn.into()),
            report: Box::new(Variable::Mvxx(0).into()),
        }
    );
}

#[test]
fn test_string_rmsc() {
    let p = parse_sstring("RMSC ATTR AERO");
    assert_eq!(
        p,
        SString::Rmsc {
            x: Box::new(Integer::Attr.into()),
            y: Box::new(Integer::Aero.into()),
        }
    );
}

#[test]
fn test_string_vois() {
    let p = parse_sstring("VOIS");
    assert_eq!(p, SString::Vois);
}

#[test]
fn test_string_rtif() {
    let p = parse_sstring("RTIF ATTR HAND");
    assert_eq!(
        p,
        SString::Rtif {
            real_time: Box::new(Integer::Attr.into()),
            format: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_string_gamn() {
    let p = parse_sstring("GAMN HAND");
    assert_eq!(
        p,
        SString::Gamn {
            previous: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_string_gnam() {
    let p = parse_sstring("GNAM");
    assert_eq!(p, SString::Gnam);
}

#[test]
fn test_string_read() {
    let p = parse_sstring("READ HAND ATTR");
    assert_eq!(
        p,
        SString::Read {
            catalogue_tag: Box::new(SString::Hand.into()),
            offset: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_string_subs() {
    let p = parse_sstring("SUBS HAND ATTR AERO");
    assert_eq!(
        p,
        SString::Subs {
            value: Box::new(SString::Hand.into()),
            start: Box::new(Integer::Attr.into()),
            count: Box::new(Integer::Aero.into())
        }
    );
}

#[test]
fn test_string_vtos() {
    let p = parse_sstring("VTOS %1");
    assert_eq!(
        p,
        SString::Vtos {
            value: Box::new(1.into())
        }
    );
}

#[test]
fn test_string_pswd() {
    let p = parse_sstring("PSWD 0");
    assert_eq!(
        p,
        SString::Pswd {
            world_index: Box::new(0.into())
        }
    );
}

#[test]
fn test_string_wnam() {
    let p = parse_sstring("WNAM");
    assert_eq!(p, SString::Wnam);
}

#[test]
fn test_string_wrld() {
    let p = parse_sstring("WRLD 0");
    assert_eq!(
        p,
        SString::Wrld {
            world_index: Box::new(0.into())
        }
    );
}

#[test]
fn test_string_wuid() {
    let p = parse_sstring("WUID");
    assert_eq!(p, SString::Wuid);
}

#[test]
fn test_prt_name() {
    let p = parse_sstring("PRT: NAME NULL 0 1");
    assert_eq!(
        p,
        SString::PrtName {
            agent: Box::new(Agent::Null.into()),
            in_or_out: Box::new(0.into()),
            port_index: Box::new(1.into())
        }
    );
}
