use super::parse_expr;
use crate::ast::{Agent, IntArg, Integer, SString, Variable};

fn parse_int_arg(content: &str) -> IntArg {
    parse_expr::<IntArg>(content)
}

fn parse_int(content: &str) -> Integer {
    let a = parse_int_arg(content);
    match a {
        IntArg::Primary(i) => i,
        _ => panic!("Expected Integer"),
    }
}

#[test]
fn test_int_arg() {
    let p = parse_int_arg("3");
    assert_eq!(p, 3.into());

    let p = parse_int_arg("3.23");
    assert_eq!(p, 3.23f32.into());

    let p = parse_int_arg("MV32");
    assert_eq!(p, Variable::Mvxx(32).into());
}

#[test]
fn test_int_attr() {
    let p = parse_int("ATTR");
    assert_eq!(p, Integer::Attr);
}

#[test]
fn test_int_base() {
    let p = parse_int("BASE");
    assert_eq!(p, Integer::Base);
}

#[test]
fn test_int_bhvr() {
    let p = parse_int("BHVR");
    assert_eq!(p, Integer::Bhvr);
}

#[test]
fn test_int_cati() {
    let p = parse_int("CATI ATTR ATTR ATTR");
    assert_eq!(
        p,
        Integer::Cati {
            family: Box::new(Integer::Attr.into()),
            genus: Box::new(Integer::Attr.into()),
            species: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_int_clac() {
    let p = parse_int("CLAC");
    assert_eq!(p, Integer::Clac);
}

#[test]
fn test_int_clik() {
    let p = parse_int("CLIK ATTR");
    assert_eq!(
        p,
        Integer::Clik {
            which_value: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_int_fmly() {
    let p = parse_int("FMLY");
    assert_eq!(p, Integer::Fmly);
}

#[test]
fn test_int_gnus() {
    let p = parse_int("GNUS");
    assert_eq!(p, Integer::Gnus);
}

#[test]
fn test_int_hght() {
    let p = parse_int("HGHT");
    assert_eq!(p, Integer::Hght);
}

#[test]
fn test_int_imsk() {
    let p = parse_int("IMSK");
    assert_eq!(p, Integer::Imsk);
}

#[test]
fn test_int_mira() {
    let p = parse_int("MIRA");
    assert_eq!(p, Integer::Mira);
}

#[test]
fn test_int_mows() {
    let p = parse_int("MOWS");
    assert_eq!(p, Integer::Mows);
}

#[test]
fn test_int_paus() {
    let p = parse_int("PAUS");
    assert_eq!(p, Integer::Paus);
}

#[test]
fn test_int_plne() {
    let p = parse_int("PLNE");
    assert_eq!(p, Integer::Plne);
}

#[test]
fn test_int_pose() {
    let p = parse_int("POSE");
    assert_eq!(p, Integer::Pose);
}

#[test]
fn test_int_puhl() {
    let p = parse_int("PUHL MOWS AERO");
    assert_eq!(
        p,
        Integer::Puhl {
            pose: Box::new(Integer::Mows.into()),
            x_or_y: Box::new(Integer::Aero.into())
        }
    );
}

#[test]
fn test_int_pupt() {
    let p = parse_int("PUPT MOWS AERO");
    assert_eq!(
        p,
        Integer::Pupt {
            pose: Box::new(Integer::Mows.into()),
            x_or_y: Box::new(Integer::Aero.into())
        }
    );
}

#[test]
fn test_int_seee() {
    let p = parse_int("SEEE NULL HELD");
    assert_eq!(
        p,
        Integer::Seee {
            first: Box::new(Agent::Null.into()),
            second: Box::new(Agent::Held.into())
        }
    );
}

#[test]
fn test_int_spcs() {
    let p = parse_int("SPCS");
    assert_eq!(p, Integer::Spcs);
}

#[test]
fn test_int_tick() {
    let p = parse_int("TICK");
    assert_eq!(p, Integer::Tick);
}

#[test]
fn test_int_totl() {
    let p = parse_int("TOTL MOWS AERO BASE");
    assert_eq!(
        p,
        Integer::Totl {
            family: Box::new(Integer::Mows.into()),
            genus: Box::new(Integer::Aero.into()),
            species: Box::new(Integer::Base.into())
        }
    );
}

#[test]
fn test_int_touc() {
    let p = parse_int("TOUC NULL HELD");
    assert_eq!(
        p,
        Integer::Touc {
            first: Box::new(Agent::Null.into()),
            second: Box::new(Agent::Held.into())
        }
    );
}

#[test]
fn test_int_visi() {
    let p = parse_int("VISI 3");
    assert_eq!(
        p,
        Integer::Visi {
            check_all_cameras: Box::new(3i32.into())
        }
    );
}

#[test]
fn test_int_wdth() {
    let p = parse_int("WDTH");
    assert_eq!(p, Integer::Wdth);
}

#[test]
fn test_int_cmrx() {
    let p = parse_int("CMRX");
    assert_eq!(p, Integer::Cmrx);
}

#[test]
fn test_int_cmry() {
    let p = parse_int("CMRY");
    assert_eq!(p, Integer::Cmry);
}

#[test]
fn test_int_loft() {
    let p = parse_int(r#"LOFT "Hello""#);
    assert_eq!(
        p,
        Integer::Loft {
            filename: Box::new(String::from("Hello").into())
        }
    );
}

#[test]
fn test_int_meta() {
    let p = parse_int("META");
    assert_eq!(p, Integer::Meta);
}

#[test]
fn test_int_snax() {
    let p = parse_int(r#"snax "Hello""#);
    assert_eq!(
        p,
        Integer::Snax {
            filename: Box::new(String::from("Hello").into())
        }
    );
}

#[test]
fn test_int_wdow() {
    let p = parse_int("WDOW");
    assert_eq!(p, Integer::Wdow);
}

#[test]
fn test_int_wndb() {
    let p = parse_int("WNDB");
    assert_eq!(p, Integer::Wndb);
}

#[test]
fn test_int_wndh() {
    let p = parse_int("WNDH");
    assert_eq!(p, Integer::Wndh);
}

#[test]
fn test_int_wndl() {
    let p = parse_int("WNDL");
    assert_eq!(p, Integer::Wndl);
}

#[test]
fn test_int_wndr() {
    let p = parse_int("WNDR");
    assert_eq!(p, Integer::Wndr);
}

#[test]
fn test_int_wndt() {
    let p = parse_int("WNDT");
    assert_eq!(p, Integer::Wndt);
}

#[test]
fn test_int_wndw() {
    let p = parse_int("WNDW");
    assert_eq!(p, Integer::Wndw);
}

#[test]
fn test_int_npgs() {
    let p = parse_int("NPGS");
    assert_eq!(p, Integer::Npgs);
}

#[test]
fn test_int_page() {
    let p = parse_int("PAGE");
    assert_eq!(p, Integer::Page);
}

#[test]
fn test_int_aslp() {
    let p = parse_int("ASLP");
    assert_eq!(p, Integer::Aslp);
}

#[test]
fn test_int_attn() {
    let p = parse_int("ATTN");
    assert_eq!(p, Integer::Attn);
}

#[test]
fn test_int_body() {
    let p = parse_int("BODY 3");
    assert_eq!(
        p,
        Integer::Body {
            body_part: Box::new(3.into())
        }
    );
}

#[test]
fn test_int_bvar() {
    let p = parse_int("BVAR");
    assert_eq!(p, Integer::Bvar);
}

#[test]
fn test_int_byit() {
    let p = parse_int("BYIT");
    assert_eq!(p, Integer::Byit);
}

#[test]
fn test_int_cage() {
    let p = parse_int("CAGE");
    assert_eq!(p, Integer::Cage);
}

#[test]
fn test_int_crea() {
    let p = parse_int("CREA NULL");
    assert_eq!(
        p,
        Integer::Crea {
            agent: Box::new(Agent::Null.into())
        }
    );
}

#[test]
fn test_int_dead() {
    let p = parse_int("DEAD");
    assert_eq!(p, Integer::Dead);
}

#[test]
fn test_int_decn() {
    let p = parse_int("DECN");
    assert_eq!(p, Integer::Decn);
}

#[test]
fn test_int_dirn() {
    let p = parse_int("DIRN");
    assert_eq!(p, Integer::Dirn);
}

#[test]
fn test_int_drea() {
    let p = parse_int("DREA");
    assert_eq!(p, Integer::Drea);
}

#[test]
fn test_int_drv() {
    let p = parse_int("DRV!");
    assert_eq!(p, Integer::Drv);
}

#[test]
fn test_int_expr() {
    let p = parse_int("EXPR");
    assert_eq!(p, Integer::Expr);
}

#[test]
fn test_int_face() {
    let p = parse_int("FACE");
    assert_eq!(p, Integer::Face);
}

#[test]
fn test_int_ins() {
    let p = parse_int("INS#");
    assert_eq!(p, Integer::Ins);
}

#[test]
fn test_int_orgi() {
    let p = parse_int("ORGI 3 ATTN");
    assert_eq!(
        p,
        Integer::Orgi {
            organ_number: Box::new(3.into()),
            data: Box::new(Integer::Attn.into())
        }
    );
}

#[test]
fn test_int_orgn() {
    let p = parse_int("ORGN");
    assert_eq!(p, Integer::Orgn);
}

#[test]
fn test_int_tage() {
    let p = parse_int("TAGE");
    assert_eq!(p, Integer::Tage);
}

#[test]
fn test_int_uncs() {
    let p = parse_int("UNCS");
    assert_eq!(p, Integer::Uncs);
}

#[test]
fn test_int_zomb() {
    let p = parse_int("ZOMB");
    assert_eq!(p, Integer::Zomb);
}

#[test]
fn test_int_code() {
    let p = parse_int("CODE");
    assert_eq!(p, Integer::Code);
}

#[test]
fn test_int_codf() {
    let p = parse_int("CODF");
    assert_eq!(p, Integer::Codf);
}

#[test]
fn test_int_codg() {
    let p = parse_int("CODG");
    assert_eq!(p, Integer::Codg);
}

#[test]
fn test_int_codp() {
    let p = parse_int("CODP");
    assert_eq!(p, Integer::Codp);
}

#[test]
fn test_int_cods() {
    let p = parse_int("CODS");
    assert_eq!(p, Integer::Cods);
}

#[test]
fn test_int_heap() {
    let p = parse_int("HEAP 3");
    assert_eq!(
        p,
        Integer::Heap {
            index: Box::new(3.into())
        }
    );
}

#[test]
fn test_int_paws() {
    let p = parse_int("PAWS");
    assert_eq!(p, Integer::Paws);
}

#[test]
fn test_int_unid() {
    let p = parse_int("UNID");
    assert_eq!(p, Integer::Unid);
}

#[test]
fn test_int_inni() {
    let p = parse_int("INNI");
    assert_eq!(p, Integer::Inni);
}

#[test]
fn test_int_inok() {
    let p = parse_int("INOK");
    assert_eq!(p, Integer::Inok);
}

#[test]
fn test_int_hist_cage() {
    let p = parse_int("HIST CAGE HAND 0");
    assert_eq!(
        p,
        Integer::HistCage {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(0.into())
        }
    );
}

#[test]
fn test_int_hist_coun() {
    let p = parse_int("HIST COUN HAND");
    assert_eq!(
        p,
        Integer::HistCoun {
            moniker: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_hist_cros() {
    let p = parse_int("HIST CROS HAND");
    assert_eq!(
        p,
        Integer::HistCros {
            moniker: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_hist_find() {
    let p = parse_int("HIST FIND HAND 1 2");
    assert_eq!(
        p,
        Integer::HistFind {
            moniker: Box::new(SString::Hand.into()),
            event_type: Box::new(1.into()),
            from_index: Box::new(2.into())
        }
    );
}

#[test]
fn test_int_hist_finr() {
    let p = parse_int("HIST FINR HAND 1 2");
    assert_eq!(
        p,
        Integer::HistFinr {
            moniker: Box::new(SString::Hand.into()),
            event_type: Box::new(1.into()),
            from_index: Box::new(2.into())
        }
    );
}

#[test]
fn test_int_hist_gend() {
    let p = parse_int("HIST GEND HAND");
    assert_eq!(
        p,
        Integer::HistGend {
            moniker: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_hist_mute() {
    let p = parse_int("HIST MUTE HAND");
    assert_eq!(
        p,
        Integer::HistMute {
            moniker: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_hist_rtim() {
    let p = parse_int("HIST RTIM HAND 3");
    assert_eq!(
        p,
        Integer::HistRtim {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(3i32.into())
        }
    );
}

#[test]
fn test_int_hist_tage() {
    let p = parse_int("HIST TAGE HAND 3");
    assert_eq!(
        p,
        Integer::HistTage {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(3i32.into())
        }
    );
}

#[test]
fn test_int_hist_type() {
    let p = parse_int("HIST TYPE HAND 3");
    assert_eq!(
        p,
        Integer::HistType {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(3i32.into())
        }
    );
}

#[test]
fn test_int_hist_vari() {
    let p = parse_int("HIST VARI HAND");
    assert_eq!(
        p,
        Integer::HistVari {
            moniker: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_hist_wtik() {
    let p = parse_int("HIST WTIK HAND 3");
    assert_eq!(
        p,
        Integer::HistWtik {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(3i32.into())
        }
    );
}

#[test]
fn test_int_hist_wuid() {
    let p = parse_int("HIST WUID HAND 3");
    assert_eq!(
        p,
        Integer::HistWuid {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(3i32.into())
        }
    );
}

#[test]
fn test_int_ooww() {
    let p = parse_int("OOWW HAND");
    assert_eq!(
        p,
        Integer::Ooww {
            moniker: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_keyd() {
    let p = parse_int("KEYD ATTR");
    assert_eq!(
        p,
        Integer::Keyd {
            key_code: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_int_mopx() {
    let p = parse_int("MOPX");
    assert_eq!(p, Integer::Mopx);
}

#[test]
fn test_int_mopy() {
    let p = parse_int("MOPY");
    assert_eq!(p, Integer::Mopy);
}

#[test]
fn test_int_pure() {
    let p = parse_int("PURE");
    assert_eq!(p, Integer::Pure);
}

#[test]
fn test_int_addm() {
    let p = parse_int("ADDM 1 2 3 4 HAND");
    assert_eq!(
        p,
        Integer::Addm {
            x: Box::new(1.into()),
            y: Box::new(2.into()),
            width: Box::new(3.into()),
            height: Box::new(4.into()),
            background: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_addr() {
    let p = parse_int("ADDR 1 2 3 4 5 6 7");
    assert_eq!(
        p,
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

#[test]
fn test_int_door() {
    let p = parse_int("DOOR 1 2");
    assert_eq!(
        p,
        Integer::Door {
            room_id1: Box::new(1.into()),
            room_id2: Box::new(2.into())
        }
    );
}

#[test]
fn test_int_down() {
    let p = parse_int("DOWN");
    assert_eq!(p, Integer::Down);
}

#[test]
fn test_int_gmap() {
    let p = parse_int("GMAP 3 4.2");
    assert_eq!(
        p,
        Integer::Gmap {
            x: Box::new(3i32.into()),
            y: Box::new(4.2f32.into())
        }
    );
}

#[test]
fn test_int_grap() {
    let p = parse_int("Grap 3 4.2");
    assert_eq!(
        p,
        Integer::Grap {
            x: Box::new(3i32.into()),
            y: Box::new(4.2f32.into())
        }
    );
}

#[test]
fn test_int_grid() {
    let p = parse_int("GRID NULL 4");
    assert_eq!(
        p,
        Integer::Grid {
            agent: Box::new(Agent::Null.into()),
            direction: Box::new(4.into())
        }
    );
}

#[test]
fn test_int_hirp() {
    let p = parse_int("HIRP -3 4 2");
    assert_eq!(
        p,
        Integer::Hirp {
            room_id: Box::new((-3i32).into()),
            ca_index: Box::new(4i32.into()),
            directions: Box::new(2i32.into())
        }
    );
}

#[test]
fn test_int_left() {
    let p = parse_int("LEFT");
    assert_eq!(p, Integer::Left);
}

#[test]
fn test_int_link() {
    let p = parse_int("LINK 1 2");
    assert_eq!(
        p,
        Integer::Link {
            room1: Box::new(1.into()),
            room2: Box::new(2.into())
        }
    );
}

#[test]
fn test_int_lorp() {
    let p = parse_int("LORP 1 2 3");
    assert_eq!(
        p,
        Integer::Lorp {
            room_id: Box::new(1.into()),
            ca_index: Box::new(2.into()),
            directions: Box::new(3.into())
        }
    );
}

#[test]
fn test_int_maph() {
    let p = parse_int("MAPH");
    assert_eq!(p, Integer::Maph);
}

#[test]
fn test_int_mapk() {
    let p = parse_int("MAPK");
    assert_eq!(p, Integer::Mapk);
}

#[test]
fn test_int_mapw() {
    let p = parse_int("MAPW");
    assert_eq!(p, Integer::Mapw);
}

#[test]
fn test_int_perm() {
    let p = parse_int("PERM");
    assert_eq!(p, Integer::Perm);
}

#[test]
fn test_int_rght() {
    let p = parse_int("RGHT");
    assert_eq!(p, Integer::Rght);
}

#[test]
fn test_int_room() {
    let p = parse_int("ROOM NULL");
    assert_eq!(
        p,
        Integer::Room {
            agent: Box::new(Agent::Null.into())
        }
    );
}

#[test]
fn test_int_rtyp() {
    let p = parse_int("RTYP ATTR");
    assert_eq!(
        p,
        Integer::Rtyp {
            room_id: Box::new(Integer::Attr.into())
        }
    );
}

#[test]
fn test_int_up() {
    let p = parse_int("_UP_");
    assert_eq!(p, Integer::Up);
}

#[test]
fn test_int_aero() {
    let p = parse_int("AERO");
    assert_eq!(p, Integer::Aero);
}

#[test]
fn test_int_elas() {
    let p = parse_int("ELAS");
    assert_eq!(p, Integer::Elas);
}

#[test]
fn test_int_fall() {
    let p = parse_int("FALL");
    assert_eq!(p, Integer::Fall);
}

#[test]
fn test_int_fric() {
    let p = parse_int("FRIC");
    assert_eq!(p, Integer::Fric);
}

#[test]
fn test_int_movs() {
    let p = parse_int("MOVS");
    assert_eq!(p, Integer::Movs);
}

#[test]
fn test_int_tmvb() {
    let p = parse_int("TMVB 3.2 1.2");
    assert_eq!(
        p,
        Integer::Tmvb {
            delta_x: Box::new(3.2f32.into()),
            delta_y: Box::new(1.2f32.into())
        }
    );
}

#[test]
fn test_int_tmvf() {
    let p = parse_int("TMVF 3.2 1.2");
    assert_eq!(
        p,
        Integer::Tmvf {
            x: Box::new(3.2f32.into()),
            y: Box::new(1.2f32.into())
        }
    );
}

#[test]
fn test_int_tmvt() {
    let p = parse_int("TMVT 3.2 1.2");
    assert_eq!(
        p,
        Integer::Tmvt {
            x: Box::new(3.2f32.into()),
            y: Box::new(1.2f32.into())
        }
    );
}

#[test]
fn test_int_wall() {
    let p = parse_int("WALL");
    assert_eq!(p, Integer::Wall);
}

#[test]
fn test_int_pray_agti() {
    let p = parse_int("PRAY AGTI HAND WNAM 0");
    assert_eq!(
        p,
        Integer::PrayAgti {
            resource_name: Box::new(SString::Hand.into()),
            integer_tag: Box::new(SString::Wnam.into()),
            default_value: Box::new(0.into())
        }
    );
}

#[test]
fn test_int_pray_coun() {
    let p = parse_int("PRAY COUN WNAM");
    assert_eq!(
        p,
        Integer::PrayCoun {
            resource_type: Box::new(SString::Wnam.into())
        }
    );
}

#[test]
fn test_int_pray_deps() {
    let p = parse_int("PRAY DEPS WNAM 0");
    assert_eq!(
        p,
        Integer::PrayDeps {
            resource_name: Box::new(SString::Wnam.into()),
            dp_install: Box::new(0.into())
        }
    );
}

#[test]
fn test_int_pray_expo() {
    let p = parse_int("PRAY EXPO WNAM");
    assert_eq!(
        p,
        Integer::PrayExpo {
            chunk_name: Box::new(SString::Wnam.into())
        }
    );
}

#[test]
fn test_int_pray_file() {
    let p = parse_int("PRAY FILE WNAM 1 2");
    assert_eq!(
        p,
        Integer::PrayFile {
            resource_name: Box::new(SString::Wnam.into()),
            resource_type: Box::new(1.into()),
            do_install: Box::new(2.into())
        }
    );
}

#[test]
fn test_int_pray_impo() {
    let p = parse_int("PRAY IMPO WNAM 1 2");
    assert_eq!(
        p,
        Integer::PrayImpo {
            moniker: Box::new(SString::Wnam.into()),
            actually_do_it: Box::new(1.into()),
            keep_file: Box::new(2.into())
        }
    );
}

#[test]
fn test_int_pray_injt() {
    let p = parse_int("PRAY INJT WNAM 1 VELX");
    assert_eq!(
        p,
        Integer::PrayInjt {
            resource_name: Box::new(SString::Wnam.into()),
            do_install: Box::new(1.into()),
            report_var: Box::new(Variable::Velx)
        }
    );
}

#[test]
fn test_int_pray_make() {
    let p = parse_int("PRAY MAKE 1 WNAM 2 HAND VELX");
    assert_eq!(
        p,
        Integer::PrayMake {
            which_journal_spot: Box::new(1.into()),
            journal_name: Box::new(SString::Wnam.into()),
            which_pray_spot: Box::new(2.into()),
            pray_name: Box::new(SString::Hand.into()),
            report_destination: Box::new(Variable::Velx)
        }
    );
}

#[test]
fn test_int_pray_size() {
    let p = parse_int("PRAY SIZE WNAM");
    assert_eq!(
        p,
        Integer::PraySize {
            resource_name: Box::new(SString::Wnam.into())
        }
    );
}

#[test]
fn test_int_pray_test() {
    let p = parse_int("PRAY TEST WNAM");
    assert_eq!(
        p,
        Integer::PrayTest {
            resource_name: Box::new(SString::Wnam.into())
        }
    );
}

#[test]
fn test_int_sorq() {
    let p = parse_int("SORQ 1 2 3 4");
    assert_eq!(
        p,
        Integer::Sorq {
            family: Box::new(1.into()),
            genus: Box::new(2.into()),
            species: Box::new(3.into()),
            event: Box::new(4.into())
        }
    );
}

#[test]
fn test_int_mute() {
    let p = parse_int("MUTE 1 2");
    assert_eq!(
        p,
        Integer::Mute {
            and_mask: Box::new(1.into()),
            eor_mask: Box::new(2.into())
        }
    );
}

#[test]
fn test_int_date() {
    let p = parse_int("DATE");
    assert_eq!(p, Integer::Date);
}

#[test]
fn test_int_dayt() {
    let p = parse_int("DAYT");
    assert_eq!(p, Integer::Dayt);
}

#[test]
fn test_int_etik() {
    let p = parse_int("ETIK");
    assert_eq!(p, Integer::Etik);
}

#[test]
fn test_int_hist_date() {
    let p = parse_int("HIST DATE 1");
    assert_eq!(
        p,
        Integer::HistDate {
            world_tick: Box::new(1.into())
        }
    );
}

#[test]
fn test_int_hist_sean() {
    let p = parse_int("HIST SEAN 1");
    assert_eq!(
        p,
        Integer::HistSean {
            world_tick: Box::new(1.into())
        }
    );
}

#[test]
fn test_int_hist_time() {
    let p = parse_int("HIST TIME 1");
    assert_eq!(
        p,
        Integer::HistTime {
            world_tick: Box::new(1.into())
        }
    );
}

#[test]
fn test_int_hist_year() {
    let p = parse_int("HIST YEAR 1");
    assert_eq!(
        p,
        Integer::HistYear {
            world_tick: Box::new(1.into())
        }
    );
}

#[test]
fn test_int_monst() {
    let p = parse_int("MONT");
    assert_eq!(p, Integer::Mont);
}

#[test]
fn test_int_msec() {
    let p = parse_int("MSEC");
    assert_eq!(p, Integer::Msec);
}

#[test]
fn test_int_race() {
    let p = parse_int("RACE");
    assert_eq!(p, Integer::Race);
}

#[test]
fn test_int_rtim() {
    let p = parse_int("RTIM");
    assert_eq!(p, Integer::Rtim);
}

#[test]
fn test_int_scol() {
    let p = parse_int("SCOL 1 2 [] [4 5]");
    assert_eq!(
        p,
        Integer::Scol {
            and_mask: Box::new(1.into()),
            eor_mask: Box::new(2.into()),
            up_speeds: Box::new(vec![].into()),
            down_speeds: Box::new(vec![4, 5].into())
        }
    );
}

#[test]
fn test_int_sean() {
    let p = parse_int("SEAN");
    assert_eq!(p, Integer::Sean);
}

#[test]
fn test_int_time() {
    let p = parse_int("TIME");
    assert_eq!(p, Integer::Time);
}

#[test]
fn test_int_wolf() {
    let p = parse_int("WOLF 1 2");
    assert_eq!(
        p,
        Integer::Wolf {
            kanga_mask: Box::new(1.into()),
            eeyore_mask: Box::new(2.into())
        }
    );
}

#[test]
fn test_int_wpau() {
    let p = parse_int("WPAU");
    assert_eq!(p, Integer::Wpau);
}

#[test]
fn test_int_wtik() {
    let p = parse_int("WTIK");
    assert_eq!(p, Integer::Wtik);
}

#[test]
fn test_int_year() {
    let p = parse_int("YEAR");
    assert_eq!(p, Integer::Year);
}

#[test]
fn test_int_char() {
    let p = parse_int("CHAR HAND 0");
    assert_eq!(
        p,
        Integer::Char {
            string: Box::new(SString::Hand.into()),
            index: Box::new(0.into())
        }
    );
}

#[test]
fn test_int_ftoi() {
    let p = parse_int("FTOI 3.2");
    assert_eq!(
        p,
        Integer::Ftoi {
            number_to_convert: Box::new(3.2f32.into())
        }
    );
}

#[test]
fn test_int_rand() {
    let p = parse_int("RAND 1 2");
    assert_eq!(
        p,
        Integer::Rand {
            value1: Box::new(1.into()),
            value2: Box::new(2.into())
        }
    );
}

#[test]
fn test_int_rean() {
    let p = parse_int("REAN HAND");
    assert_eq!(
        p,
        Integer::Rean {
            catalogue_tag: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_reaq() {
    let p = parse_int("REAQ HAND");
    assert_eq!(
        p,
        Integer::Reaq {
            catalogue_tag: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_stoi() {
    let p = parse_int("STOI HAND");
    assert_eq!(
        p,
        Integer::Stoi {
            value: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_strl() {
    let p = parse_int("STRL HAND");
    assert_eq!(
        p,
        Integer::Strl {
            value: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_type() {
    let p = parse_int("TYPE HAND");
    assert_eq!(
        p,
        Integer::Type {
            something: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_vmjr() {
    let p = parse_int("VMJR");
    assert_eq!(p, Integer::Vmjr);
}

#[test]
fn test_int_vmnr() {
    let p = parse_int("VMNR");
    assert_eq!(p, Integer::Vmnr);
}

#[test]
fn test_int_cabb() {
    let p = parse_int("CABB");
    assert_eq!(p, Integer::Cabb);
}

#[test]
fn test_int_cabl() {
    let p = parse_int("CABL");
    assert_eq!(p, Integer::Cabl);
}

#[test]
fn test_int_cabp() {
    let p = parse_int("CABP");
    assert_eq!(p, Integer::Cabp);
}

#[test]
fn test_int_cabr() {
    let p = parse_int("CABR");
    assert_eq!(p, Integer::Cabr);
}

#[test]
fn test_int_cabt() {
    let p = parse_int("CABT");
    assert_eq!(p, Integer::Cabt);
}

#[test]
fn test_int_cabv() {
    let p = parse_int("CABV");
    assert_eq!(p, Integer::Cabv);
}

#[test]
fn test_int_nwld() {
    let p = parse_int("NWLD");
    assert_eq!(p, Integer::Nwld);
}

#[test]
fn test_int_wnti() {
    let p = parse_int("WNTI HAND");
    assert_eq!(
        p,
        Integer::Wnti {
            world: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_int_prt_itot() {
    let p = parse_int("PRT: ITOT");
    assert_eq!(p, Integer::PrtItot);
}

#[test]
fn test_int_prt_from() {
    let p = parse_int("PRT: FROM 0");
    assert_eq!(
        p,
        Integer::PrtFrom {
            input_port: Box::new(0.into())
        }
    );
}

#[test]
fn test_int_prt_otot() {
    let p = parse_int("PRT: OTOT");
    assert_eq!(p, Integer::PrtOtot);
}
