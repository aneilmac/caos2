use super::*;
use crate::{
    ast::Agent,
    ast::Float,
    ast::{SString, Variable},
    parser::CaosParser,
};
use pest::Parser;

#[test]
fn test_int_arg() {
    for p in CaosParser::parse(Rule::int_arg, "3").expect("Parsed") {
        assert_eq!(
            parse_int_arg(p).expect("Parsed variable"),
            IntArg::Primary(Integer::Literal(3)),
        );
    }
    for p in CaosParser::parse(Rule::int_arg, "3.23").expect("Parsed") {
        assert_eq!(
            parse_int_arg(p).expect("Parsed variable"),
            IntArg::Castable(Float::Literal(3.23.into())),
        );
    }
    for p in CaosParser::parse(Rule::int_arg, "MV32").expect("Parsed") {
        assert_eq!(
            parse_int_arg(p).expect("Parsed variable"),
            IntArg::Variable(Variable::Mvxx(32)),
        );
    }
}

#[test]
fn test_int_attr() {
    for p in CaosParser::parse(Rule::int, "ATTR").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Attr);
    }
}

#[test]
fn test_int_base() {
    for p in CaosParser::parse(Rule::int, "BASE").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Base);
    }
}

#[test]
fn test_int_bhvr() {
    for p in CaosParser::parse(Rule::int, "bhvr").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Bhvr);
    }
}

#[test]
fn test_int_cati() {
    for p in CaosParser::parse(Rule::int, "CATI ATTR ATTR ATTR").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Cati {
                family: Box::new(IntArg::Primary(Integer::Attr)),
                genus: Box::new(IntArg::Primary(Integer::Attr)),
                species: Box::new(IntArg::Primary(Integer::Attr))
            },
        );
    }
}

#[test]
fn test_int_clac() {
    for p in CaosParser::parse(Rule::int, "clac").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Clac);
    }
}

#[test]
fn test_int_clik() {
    for p in CaosParser::parse(Rule::int, "clik attr").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Clik {
                which_value: Box::new(Integer::Attr.into())
            }
        );
    }
}

#[test]
fn test_int_fmly() {
    for p in CaosParser::parse(Rule::int, "FMLY").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Fmly);
    }
}

#[test]
fn test_int_gnus() {
    for p in CaosParser::parse(Rule::int, "gnus").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Gnus);
    }
}

#[test]
fn test_int_hght() {
    for p in CaosParser::parse(Rule::int, "HGHT").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Hght);
    }
}

#[test]
fn test_int_imsk() {
    for p in CaosParser::parse(Rule::int, "imsk").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Imsk);
    }
}

#[test]
fn test_int_mira() {
    for p in CaosParser::parse(Rule::int, "MIRA").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Mira);
    }
}

#[test]
fn test_int_mows() {
    for p in CaosParser::parse(Rule::int, "mows").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Mows);
    }
}

#[test]
fn test_int_paus() {
    for p in CaosParser::parse(Rule::int, "paus").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Paus);
    }
}

#[test]
fn test_int_plne() {
    for p in CaosParser::parse(Rule::int, "plne").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Plne);
    }
}

#[test]
fn test_int_pose() {
    for p in CaosParser::parse(Rule::int, "POSE").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Pose);
    }
}

#[test]
fn test_int_puhl() {
    for p in CaosParser::parse(Rule::int, "PUHL mows Aero").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Puhl {
                pose: Box::new(Integer::Mows.into()),
                x_or_y: Box::new(Integer::Aero.into())
            }
        );
    }
}

#[test]
fn test_int_pupt() {
    for p in CaosParser::parse(Rule::int, "PUPT mows Aero").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Pupt {
                pose: Box::new(Integer::Mows.into()),
                x_or_y: Box::new(Integer::Aero.into())
            }
        );
    }
}

#[test]
fn test_int_seee() {
    for p in CaosParser::parse(Rule::int, "SEEE null Held").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Seee {
                first: Box::new(Agent::Null.into()),
                second: Box::new(Agent::Held.into())
            }
        );
    }
}

#[test]
fn test_int_spcs() {
    for p in CaosParser::parse(Rule::int, "SPCS").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Spcs);
    }
}

#[test]
fn test_int_tick() {
    for p in CaosParser::parse(Rule::int, "TICK").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Tick);
    }
}

#[test]
fn test_int_totl() {
    for p in CaosParser::parse(Rule::int, "TOTL MOWS AERO BASE").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Totl {
                family: Box::new(Integer::Mows.into()),
                genus: Box::new(Integer::Aero.into()),
                species: Box::new(Integer::Base.into())
            }
        );
    }
}

#[test]
fn test_int_touc() {
    for p in CaosParser::parse(Rule::int, "TOUC NULL HELD").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Touc {
                first: Box::new(Agent::Null.into()),
                second: Box::new(Agent::Held.into())
            }
        );
    }
}

#[test]
fn test_int_visi() {
    for p in CaosParser::parse(Rule::int, "VISI 3").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Visi {
                check_all_cameras: Box::new(3i32.into())
            }
        );
    }
}

#[test]
fn test_int_wdth() {
    for p in CaosParser::parse(Rule::int, "WDTH").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wdth);
    }
}

#[test]
fn test_int_cmrx() {
    for p in CaosParser::parse(Rule::int, "CMRX").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cmrx);
    }
}

#[test]
fn test_int_cmry() {
    for p in CaosParser::parse(Rule::int, "CMRY").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cmry);
    }
}

#[test]
fn test_int_loft() {
    for p in CaosParser::parse(Rule::int, r#"LOFT "Hello""#).expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Loft {
                filename: Box::new(String::from("Hello").into())
            }
        );
    }
}

#[test]
fn test_int_meta() {
    for p in CaosParser::parse(Rule::int, "META").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Meta);
    }
}

#[test]
fn test_int_snax() {
    for p in CaosParser::parse(Rule::int, r#"snax "Hello""#).expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Snax {
                filename: Box::new(String::from("Hello").into())
            }
        );
    }
}

#[test]
fn test_int_wdow() {
    for p in CaosParser::parse(Rule::int, "wdow").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wdow);
    }
}

#[test]
fn test_int_wndb() {
    for p in CaosParser::parse(Rule::int, "wndb").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wndb);
    }
}

#[test]
fn test_int_wndh() {
    for p in CaosParser::parse(Rule::int, "wndh").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wndh);
    }
}

#[test]
fn test_int_wndl() {
    for p in CaosParser::parse(Rule::int, "wndl").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wndl);
    }
}

#[test]
fn test_int_wndr() {
    for p in CaosParser::parse(Rule::int, "wndr").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wndr);
    }
}

#[test]
fn test_int_wndt() {
    for p in CaosParser::parse(Rule::int, "wndt").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wndt);
    }
}

#[test]
fn test_int_wndw() {
    for p in CaosParser::parse(Rule::int, "wndw").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wndw);
    }
}

#[test]
fn test_int_npgs() {
    for p in CaosParser::parse(Rule::int, "NPGS").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Npgs);
    }
}

#[test]
fn test_int_page() {
    for p in CaosParser::parse(Rule::int, "PAGE").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Page);
    }
}

#[test]
fn test_int_aslp() {
    for p in CaosParser::parse(Rule::int, "aslp").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Aslp);
    }
}

#[test]
fn test_int_attn() {
    for p in CaosParser::parse(Rule::int, "ATTN").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Attn);
    }
}

#[test]
fn test_int_body() {
    for p in CaosParser::parse(Rule::int, "BODY 3").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Body {
                body_part: Box::new(3.into())
            }
        );
    }
}

#[test]
fn test_int_bvar() {
    for p in CaosParser::parse(Rule::int, "BVAR").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Bvar);
    }
}

#[test]
fn test_int_byit() {
    for p in CaosParser::parse(Rule::int, "BYIT").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Byit);
    }
}

#[test]
fn test_int_cage() {
    for p in CaosParser::parse(Rule::int, "cage").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cage);
    }
}

#[test]
fn test_int_crea() {
    for p in CaosParser::parse(Rule::int, "CREA NULL").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Crea {
                agent: Box::new(Agent::Null.into())
            }
        );
    }
}

#[test]
fn test_int_dead() {
    for p in CaosParser::parse(Rule::int, "DEAD").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Dead);
    }
}

#[test]
fn test_int_decn() {
    for p in CaosParser::parse(Rule::int, "decn").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Decn);
    }
}

#[test]
fn test_int_dirn() {
    for p in CaosParser::parse(Rule::int, "DIRN").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Dirn);
    }
}

#[test]
fn test_int_drea() {
    for p in CaosParser::parse(Rule::int, "drea").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Drea);
    }
}

#[test]
fn test_int_drv() {
    for p in CaosParser::parse(Rule::int, "DRV!").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Drv);
    }
}

#[test]
fn test_int_expr() {
    for p in CaosParser::parse(Rule::int, "EXPR").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Expr);
    }
}

#[test]
fn test_int_face() {
    for p in CaosParser::parse(Rule::int, "FACE").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Face);
    }
}

#[test]
fn test_int_ins() {
    for p in CaosParser::parse(Rule::int, "Ins#").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Ins);
    }
}

#[test]
fn test_int_orgi() {
    for p in CaosParser::parse(Rule::int, "orgi 3 ATTN").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Orgi {
                organ_number: Box::new(3.into()),
                data: Box::new(Integer::Attn.into())
            }
        );
    }
}

#[test]
fn test_int_orgn() {
    for p in CaosParser::parse(Rule::int, "orgn").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Orgn);
    }
}

#[test]
fn test_int_tage() {
    for p in CaosParser::parse(Rule::int, "tage").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Tage);
    }
}

#[test]
fn test_int_uncs() {
    for p in CaosParser::parse(Rule::int, "UNCS").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Uncs);
    }
}

#[test]
fn test_int_zomb() {
    for p in CaosParser::parse(Rule::int, "zomb").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Zomb);
    }
}

#[test]
fn test_int_code() {
    for p in CaosParser::parse(Rule::int, "code").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Code);
    }
}

#[test]
fn test_int_codf() {
    for p in CaosParser::parse(Rule::int, "codf").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Codf);
    }
}

#[test]
fn test_int_codg() {
    for p in CaosParser::parse(Rule::int, "codg").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Codg);
    }
}

#[test]
fn test_int_codp() {
    for p in CaosParser::parse(Rule::int, "codp").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Codp);
    }
}

#[test]
fn test_int_cods() {
    for p in CaosParser::parse(Rule::int, "cods").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cods);
    }
}

#[test]
fn test_int_heap() {
    for p in CaosParser::parse(Rule::int, "HEAP 3").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Heap {
                index: Box::new(3.into())
            }
        );
    }
}

#[test]
fn test_int_paws() {
    for p in CaosParser::parse(Rule::int, "paws").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Paws);
    }
}

#[test]
fn test_int_unid() {
    for p in CaosParser::parse(Rule::int, "UNID").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Unid);
    }
}

#[test]
fn test_int_inni() {
    for p in CaosParser::parse(Rule::int, "inni").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Inni);
    }
}

#[test]
fn test_int_inok() {
    for p in CaosParser::parse(Rule::int, "INOK").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Inok);
    }
}

#[test]
fn test_int_hist_cage() {
    for p in CaosParser::parse(Rule::int, "HIST CAGE HAND 0").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistCage {
                moniker: Box::new(SString::Hand.into()),
                event_no: Box::new(0.into())
            }
        );
    }
}

#[test]
fn test_int_hist_coun() {
    for p in CaosParser::parse(Rule::int, "HIST COUN HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistCoun {
                moniker: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_hist_cros() {
    for p in CaosParser::parse(Rule::int, "HIST CROS HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistCros {
                moniker: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_hist_find() {
    for p in CaosParser::parse(Rule::int, "HIST FIND HAND 1 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistFind {
                moniker: Box::new(SString::Hand.into()),
                event_type: Box::new(1.into()),
                from_index: Box::new(2.into())
            }
        );
    }
}

#[test]
fn test_int_hist_finr() {
    for p in CaosParser::parse(Rule::int, "HIST FINR HAND 1 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistFinr {
                moniker: Box::new(SString::Hand.into()),
                event_type: Box::new(1.into()),
                from_index: Box::new(2.into())
            }
        );
    }
}

#[test]
fn test_int_hist_gend() {
    for p in CaosParser::parse(Rule::int, "HIST GEND HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistGend {
                moniker: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_hist_mute() {
    for p in CaosParser::parse(Rule::int, "HIST MUTE HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistMute {
                moniker: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_hist_prev() {
    for p in CaosParser::parse(Rule::int, "HIST PREV HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistPrev {
                moniker: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_hist_rtim() {
    for p in CaosParser::parse(Rule::int, "HIST RTIM HAND 3").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistRtim {
                moniker: Box::new(SString::Hand.into()),
                event_no: Box::new(3i32.into())
            }
        );
    }
}

#[test]
fn test_int_hist_tage() {
    for p in CaosParser::parse(Rule::int, "hist tage HAND 3").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistTage {
                moniker: Box::new(SString::Hand.into()),
                event_no: Box::new(3i32.into())
            }
        );
    }
}

#[test]
fn test_int_hist_type() {
    for p in CaosParser::parse(Rule::int, "hist type HAND 3").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistType {
                moniker: Box::new(SString::Hand.into()),
                event_no: Box::new(3i32.into())
            }
        );
    }
}

#[test]
fn test_int_hist_vari() {
    for p in CaosParser::parse(Rule::int, "hist vari HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistVari {
                moniker: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_hist_wtik() {
    for p in CaosParser::parse(Rule::int, "hist wtik HAND 3").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistWtik {
                moniker: Box::new(SString::Hand.into()),
                event_no: Box::new(3i32.into())
            }
        );
    }
}

#[test]
fn test_int_hist_wuid() {
    for p in CaosParser::parse(Rule::int, "hist wuid HAND 3").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistWuid {
                moniker: Box::new(SString::Hand.into()),
                event_no: Box::new(3i32.into())
            }
        );
    }
}

#[test]
fn test_int_ooww() {
    for p in CaosParser::parse(Rule::int, "OOWW HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Ooww {
                moniker: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_keyd() {
    for p in CaosParser::parse(Rule::int, "KEYD ATTR").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Keyd {
                key_code: Box::new(Integer::Attr.into())
            }
        );
    }
}

#[test]
fn test_int_mopx() {
    for p in CaosParser::parse(Rule::int, "MOPX").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Mopx);
    }
}

#[test]
fn test_int_mopy() {
    for p in CaosParser::parse(Rule::int, "MOpY").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Mopy);
    }
}

#[test]
fn test_int_pure() {
    for p in CaosParser::parse(Rule::int, "PURE").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Pure);
    }
}

#[test]
fn test_int_addm() {
    for p in CaosParser::parse(Rule::int, "ADDM 1 2 3 4 HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Addm {
                x: Box::new(1.into()),
                y: Box::new(2.into()),
                width: Box::new(3.into()),
                height: Box::new(4.into()),
                background: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_addr() {
    for p in CaosParser::parse(Rule::int, "ADDR 1 2 3 4 5 6 7").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Addr {
                metaroom_id: Box::new(1.into()),
                x_left: Box::new(2.into()),
                y_right: Box::new(3.into()),
                y_left_ceiling: Box::new(4.into()),
                y_right_ceiling: Box::new(5.into()),
                y_left_floor: Box::new(6.into()),
                y_right_floor: Box::new(7.into())
            }
        );
    }
}

#[test]
fn test_int_door() {
    for p in CaosParser::parse(Rule::int, "DOOR 1 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Door {
                room_id1: Box::new(1.into()),
                room_id2: Box::new(2.into())
            }
        );
    }
}

#[test]
fn test_int_down() {
    for p in CaosParser::parse(Rule::int, "DOWN").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Down);
    }
}

#[test]
fn test_int_gmap() {
    for p in CaosParser::parse(Rule::int, "GMAP 3 4.2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Gmap {
                x: Box::new(3i32.into()),
                y: Box::new(4.2f32.into())
            }
        );
    }
}

#[test]
fn test_int_grap() {
    for p in CaosParser::parse(Rule::int, "GRAP 3 4.2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Grap {
                x: Box::new(3i32.into()),
                y: Box::new(4.2f32.into())
            }
        );
    }
}

#[test]
fn test_int_grid() {
    for p in CaosParser::parse(Rule::int, "GRID NULL 4").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Grid {
                agent: Box::new(Agent::Null.into()),
                direction: Box::new(4.into())
            }
        );
    }
}

#[test]
fn test_int_hirp() {
    for p in CaosParser::parse(Rule::int, "HIRP -3 4 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Hirp {
                room_id: Box::new((-3i32).into()),
                ca_index: Box::new(4i32.into()),
                directions: Box::new(2i32.into())
            }
        );
    }
}

#[test]
fn test_int_left() {
    for p in CaosParser::parse(Rule::int, "left").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Left);
    }
}

#[test]
fn test_int_link() {
    for p in CaosParser::parse(Rule::int, "LINK 1 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Link {
                room1: Box::new(1.into()),
                room2: Box::new(2.into())
            }
        );
    }
}

#[test]
fn test_int_lorp() {
    for p in CaosParser::parse(Rule::int, "LORP 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Lorp {
                room_id: Box::new(1.into()),
                ca_index: Box::new(2.into()),
                directions: Box::new(3.into())
            }
        );
    }
}

#[test]
fn test_int_maph() {
    for p in CaosParser::parse(Rule::int, "maph").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Maph);
    }
}

#[test]
fn test_int_mapk() {
    for p in CaosParser::parse(Rule::int, "mapk").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Mapk);
    }
}

#[test]
fn test_int_mapw() {
    for p in CaosParser::parse(Rule::int, "mapw").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Mapw);
    }
}

#[test]
fn test_int_perm() {
    for p in CaosParser::parse(Rule::int, "perm").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Perm);
    }
}

#[test]
fn test_int_rght() {
    for p in CaosParser::parse(Rule::int, "RGHT").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Rght);
    }
}

#[test]
fn test_int_room() {
    for p in CaosParser::parse(Rule::int, "ROOM NULL").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Room {
                agent: Box::new(Agent::Null.into())
            }
        );
    }
}

#[test]
fn test_int_rtyp() {
    for p in CaosParser::parse(Rule::int, "RTYP attr").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Rtyp {
                room_id: Box::new(Integer::Attr.into())
            }
        );
    }
}

#[test]
fn test_int_up() {
    for p in CaosParser::parse(Rule::int, "_up_").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Up);
    }
}

#[test]
fn test_int_aero() {
    for p in CaosParser::parse(Rule::int, "aero").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Aero);
    }
}

#[test]
fn test_int_elas() {
    for p in CaosParser::parse(Rule::int, "elas").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Elas);
    }
}

#[test]
fn test_int_fall() {
    for p in CaosParser::parse(Rule::int, "FALL").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Fall);
    }
}

#[test]
fn test_int_fric() {
    for p in CaosParser::parse(Rule::int, "FRIC").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Fric);
    }
}

#[test]
fn test_int_movs() {
    for p in CaosParser::parse(Rule::int, "movs").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Movs);
    }
}

#[test]
fn test_int_tmvb() {
    for p in CaosParser::parse(Rule::int, "tmvb 3.2 1.2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Tmvb {
                delta_x: Box::new(3.2f32.into()),
                delta_y: Box::new(1.2f32.into())
            }
        );
    }
}

#[test]
fn test_int_tmvf() {
    for p in CaosParser::parse(Rule::int, "tmvf 3.2 1.2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Tmvf {
                x: Box::new(3.2f32.into()),
                y: Box::new(1.2f32.into())
            }
        );
    }
}

#[test]
fn test_int_tmvt() {
    for p in CaosParser::parse(Rule::int, "tmvt 3.2 1.2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Tmvt {
                x: Box::new(3.2f32.into()),
                y: Box::new(1.2f32.into())
            }
        );
    }
}

#[test]
fn test_int_wall() {
    for p in CaosParser::parse(Rule::int, "WALL").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wall);
    }
}

#[test]
fn test_int_pray_agti() {
    for p in CaosParser::parse(Rule::int, "PRAY AGTI hand wnam 0").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrayAgti {
                resource_name: Box::new(SString::Hand.into()),
                integer_tag: Box::new(SString::Wnam.into()),
                default_value: Box::new(0.into())
            }
        );
    }
}

#[test]
fn test_int_pray_coun() {
    for p in CaosParser::parse(Rule::int, "PRAY COUN wnam").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrayCoun {
                resource_type: Box::new(SString::Wnam.into())
            }
        );
    }
}

#[test]
fn test_int_pray_deps() {
    for p in CaosParser::parse(Rule::int, "PRAY DEPS wnam 0").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrayDeps {
                resource_name: Box::new(SString::Wnam.into()),
                dp_install: Box::new(0.into())
            }
        );
    }
}

#[test]
fn test_int_pray_expo() {
    for p in CaosParser::parse(Rule::int, "PRAY EXPO WNAM").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrayExpo {
                chunk_name: Box::new(SString::Wnam.into())
            }
        );
    }
}

#[test]
fn test_int_pray_file() {
    for p in CaosParser::parse(Rule::int, "PRAY FILE WNAM 1 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrayFile {
                resource_name: Box::new(SString::Wnam.into()),
                resource_type: Box::new(1.into()),
                do_install: Box::new(2.into())
            }
        );
    }
}

#[test]
fn test_int_pray_impo() {
    for p in CaosParser::parse(Rule::int, "PRAY impo WNAM 1 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrayImpo {
                moniker: Box::new(SString::Wnam.into()),
                actually_do_it: Box::new(1.into()),
                keep_file: Box::new(2.into())
            }
        );
    }
}

#[test]
fn test_int_pray_injt() {
    for p in CaosParser::parse(Rule::int, "PRAY injt WNAM 1 VELX").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrayInjt {
                resource_name: Box::new(SString::Wnam.into()),
                do_install: Box::new(1.into()),
                report_var: Box::new(Variable::Velx)
            }
        );
    }
}

#[test]
fn test_int_pray_make() {
    for p in CaosParser::parse(Rule::int, "PRAY MAKE 1 WNAM 2 HAND VELX").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrayMake {
                which_journal_spot: Box::new(1.into()),
                journal_name: Box::new(SString::Wnam.into()),
                which_pray_spot: Box::new(2.into()),
                pray_name: Box::new(SString::Hand.into()),
                report_destination: Box::new(Variable::Velx)
            }
        );
    }
}

#[test]
fn test_int_pray_size() {
    for p in CaosParser::parse(Rule::int, "PRAY SIZE WNAM").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PraySize {
                resource_name: Box::new(SString::Wnam.into())
            }
        );
    }
}

#[test]
fn test_int_pray_test() {
    for p in CaosParser::parse(Rule::int, "PRAY TEST WNAM").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrayTest {
                resource_name: Box::new(SString::Wnam.into())
            }
        );
    }
}

#[test]
fn test_int_sorq() {
    for p in CaosParser::parse(Rule::int, "SORQ 1 2 3 4").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Sorq {
                family: Box::new(1.into()),
                genus: Box::new(2.into()),
                species: Box::new(3.into()),
                event: Box::new(4.into())
            }
        );
    }
}

#[test]
fn test_int_mute() {
    for p in CaosParser::parse(Rule::int, "mute 1 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Mute {
                and_mask: Box::new(1.into()),
                eor_mask: Box::new(2.into())
            }
        );
    }
}

#[test]
fn test_int_date() {
    for p in CaosParser::parse(Rule::int, "DATE").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Date);
    }
}

#[test]
fn test_int_dayt() {
    for p in CaosParser::parse(Rule::int, "dayt").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Dayt);
    }
}

#[test]
fn test_int_etik() {
    for p in CaosParser::parse(Rule::int, "ETIK").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Etik);
    }
}

#[test]
fn test_int_hist_date() {
    for p in CaosParser::parse(Rule::int, "hist date 1").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistDate {
                world_tick: Box::new(1.into())
            }
        );
    }
}

#[test]
fn test_int_hist_sean() {
    for p in CaosParser::parse(Rule::int, "hist sean 1").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistSean {
                world_tick: Box::new(1.into())
            }
        );
    }
}

#[test]
fn test_int_hist_time() {
    for p in CaosParser::parse(Rule::int, "hist time 1").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistTime {
                world_tick: Box::new(1.into())
            }
        );
    }
}

#[test]
fn test_int_hist_year() {
    for p in CaosParser::parse(Rule::int, "hist year 1").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::HistYear {
                world_tick: Box::new(1.into())
            }
        );
    }
}

#[test]
fn test_int_monst() {
    for p in CaosParser::parse(Rule::int, "MONT").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Mont);
    }
}

#[test]
fn test_int_msec() {
    for p in CaosParser::parse(Rule::int, "MSEC").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Msec);
    }
}

#[test]
fn test_int_race() {
    for p in CaosParser::parse(Rule::int, "RACE").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Race);
    }
}

#[test]
fn test_int_rtim() {
    for p in CaosParser::parse(Rule::int, "rtim").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Rtim);
    }
}

#[test]
fn test_int_scol() {
    for p in CaosParser::parse(Rule::int, "SCOL 1 2 [] [4 5]").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Scol {
                and_mask: Box::new(1.into()),
                eor_mask: Box::new(2.into()),
                up_speeds: Box::new(vec![].into()),
                down_speeds: Box::new(vec![4, 5].into())
            }
        );
    }
}

#[test]
fn test_int_sean() {
    for p in CaosParser::parse(Rule::int, "SEAN").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Sean);
    }
}

#[test]
fn test_int_time() {
    for p in CaosParser::parse(Rule::int, "TIME").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Time);
    }
}

#[test]
fn test_int_wolf() {
    for p in CaosParser::parse(Rule::int, "wolf 1 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Wolf {
                kanga_mask: Box::new(1.into()),
                eeyore_mask: Box::new(2.into())
            }
        );
    }
}

#[test]
fn test_int_wpau() {
    for p in CaosParser::parse(Rule::int, "WPAU").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wpau);
    }
}

#[test]
fn test_int_wtik() {
    for p in CaosParser::parse(Rule::int, "WTIK").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Wtik);
    }
}

#[test]
fn test_int_year() {
    for p in CaosParser::parse(Rule::int, "YEAR").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Year);
    }
}

#[test]
fn test_int_char() {
    for p in CaosParser::parse(Rule::int, "CHAR HAND 0").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Char {
                string: Box::new(SString::Hand.into()),
                index: Box::new(0.into())
            }
        );
    }
}

#[test]
fn test_int_ftoi() {
    for p in CaosParser::parse(Rule::int, "FTOI 3.2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Ftoi {
                number_to_convert: Box::new(3.2f32.into())
            }
        );
    }
}

#[test]
fn test_int_rand() {
    for p in CaosParser::parse(Rule::int, "RAND 1 2").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Rand {
                value1: Box::new(1.into()),
                value2: Box::new(2.into())
            }
        );
    }
}

#[test]
fn test_int_rean() {
    for p in CaosParser::parse(Rule::int, "REAN HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Rean {
                catalogue_tag: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_reaq() {
    for p in CaosParser::parse(Rule::int, "REAQ HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Reaq {
                catalogue_tag: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_stoi() {
    for p in CaosParser::parse(Rule::int, "STOI HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Stoi {
                value: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_strl() {
    for p in CaosParser::parse(Rule::int, "STRL HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Strl {
                value: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_type() {
    for p in CaosParser::parse(Rule::int, "TYPE HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Type {
                something: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_vmjr() {
    for p in CaosParser::parse(Rule::int, "VMJR").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Vmjr);
    }
}

#[test]
fn test_int_vmmr() {
    for p in CaosParser::parse(Rule::int, "VMNR").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Vmnr);
    }
}

#[test]
fn test_int_cabb() {
    for p in CaosParser::parse(Rule::int, "CABB").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cabb);
    }
}

#[test]
fn test_int_cabl() {
    for p in CaosParser::parse(Rule::int, "CABL").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cabl);
    }
}

#[test]
fn test_int_cabp() {
    for p in CaosParser::parse(Rule::int, "CABP").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cabp);
    }
}

#[test]
fn test_int_cabr() {
    for p in CaosParser::parse(Rule::int, "CABR").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cabr);
    }
}

#[test]
fn test_int_cabt() {
    for p in CaosParser::parse(Rule::int, "CABT").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cabt);
    }
}

#[test]
fn test_int_cabv() {
    for p in CaosParser::parse(Rule::int, "CABV").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Cabv);
    }
}

#[test]
fn test_int_nwld() {
    for p in CaosParser::parse(Rule::int, "NWLD").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::Nwld);
    }
}

#[test]
fn test_int_wnti() {
    for p in CaosParser::parse(Rule::int, "WNTI HAND").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::Wnti {
                world: Box::new(SString::Hand.into())
            }
        );
    }
}

#[test]
fn test_int_prt_itot() {
    for p in CaosParser::parse(Rule::int, "PRT: ITOT").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::PrtItot);
    }
}

#[test]
fn test_int_prt_from() {
    for p in CaosParser::parse(Rule::int, "PRT: FROM 0").expect("Parsed") {
        assert_eq!(
            parse_int(p).expect("Parsed variable"),
            Integer::PrtFrom {
                input_port: Box::new(0.into())
            }
        );
    }
}

#[test]
fn test_int_prt_otot() {
    for p in CaosParser::parse(Rule::int, "PRT: OTOT").expect("Parsed") {
        assert_eq!(parse_int(p).expect("Parsed variable"), Integer::PrtOtot);
    }
}
