use crate::{
    ast::{Agent, Condition, SString, Variable},
    parser::CaosParser,
};
use pest::Parser;

use super::*;

#[test]
fn test_command_anim() {
    assert_eq!(
        parse_cmnd("ANIM [1 2]"),
        Command::Anim {
            pose_list: Box::new(vec![1, 2].into()),
        }
    );
}

#[test]
fn test_command_anms() {
    assert_eq!(
        parse_cmnd("ANMS HAND"),
        Command::Anms {
            anim_string: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_attr() {
    assert_eq!(
        parse_cmnd("ATTR 0"),
        Command::Attr {
            attributes: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_base() {
    assert_eq!(
        parse_cmnd("BASE 0"),
        Command::Base {
            index: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_bhvr() {
    assert_eq!(
        parse_cmnd("BHVR 0"),
        Command::Bhvr {
            permissions: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_frat() {
    assert_eq!(
        parse_cmnd("FRAT 0"),
        Command::Frat {
            framerate: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_gait() {
    assert_eq!(
        parse_cmnd("GAIT 0"),
        Command::Gait {
            gait_number: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_gall() {
    assert_eq!(
        parse_cmnd("GALL HAND 0"),
        Command::Gall {
            sprite_file: Box::new(SString::Hand.into()),
            first_image: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_hand() {
    assert_eq!(
        parse_cmnd("HAND HAND"),
        Command::Hand {
            name_for_the_hand: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_kill() {
    assert_eq!(
        parse_cmnd("KILL NULL"),
        Command::Kill {
            agent: Box::new(Agent::Null.into()),
        }
    );
}

#[test]
fn test_command_mesg_writ() {
    assert_eq!(
        parse_cmnd("MESG WRIT NULL 0"),
        Command::MesgWrit {
            agent: Box::new(Agent::Null.into()),
            message_id: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_mesg_wrt() {
    assert_eq!(
        parse_cmnd("MESG WRT+ NULL 0 1 2 3"),
        Command::MesgWritPlus {
            agent: Box::new(Agent::Null.into()),
            message_id: Box::new(0.into()),
            param_1: Box::new(1.into()),
            param_2: Box::new(2.into()),
            delay: Box::new(3.into()),
        }
    );
}

#[test]
fn test_command_mira() {
    assert_eq!(
        parse_cmnd("MIRA 0"),
        Command::Mira {
            on_off: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_new_simp() {
    assert_eq!(
        parse_cmnd("NEW: SIMP 0 1 2 HAND 3 4 5"),
        Command::NewSimp {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            sprite_file: Box::new(SString::Hand.into()),
            image_count: Box::new(3.into()),
            first_image: Box::new(4.into()),
            plane: Box::new(5.into()),
        }
    );
}

#[test]
fn test_command_nohh() {
    assert_eq!(parse_cmnd("NOHH"), Command::Nohh);
}

#[test]
fn test_command_over() {
    assert_eq!(parse_cmnd("OVER"), Command::Over);
}

#[test]
fn test_command_paus() {
    assert_eq!(
        parse_cmnd("PAUS 0"),
        Command::Paus {
            paused: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_plne() {
    assert_eq!(
        parse_cmnd("PLNE 0"),
        Command::Plne {
            plane: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_pose() {
    assert_eq!(
        parse_cmnd("POSE 0"),
        Command::Pose {
            pose: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_puhl() {
    assert_eq!(
        parse_cmnd("PUHL 0 1 2"),
        Command::Puhl {
            pose: Box::new(0.into()),
            x: Box::new(1.into()),
            y: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_pupt() {
    assert_eq!(
        parse_cmnd("PUPT 0 1 2"),
        Command::Pupt {
            pose: Box::new(0.into()),
            x: Box::new(1.into()),
            y: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_rnge() {
    assert_eq!(
        parse_cmnd("RNGE 0.0"),
        Command::Rnge {
            distance: Box::new(0.0f32.into()),
        }
    );
}

#[test]
fn test_command_rtar() {
    assert_eq!(
        parse_cmnd("RTAR 0 1 2"),
        Command::Rtar {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_show() {
    assert_eq!(
        parse_cmnd("SHOW 0"),
        Command::Show {
            visibility: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_star() {
    assert_eq!(
        parse_cmnd("STAR 0 1 2"),
        Command::Star {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_tick() {
    assert_eq!(
        parse_cmnd("TICK 0"),
        Command::Tick {
            tick_rate: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_tint() {
    assert_eq!(
        parse_cmnd("TINT 0 1 2 3 4"),
        Command::Tint {
            red_tint: Box::new(0.into()),
            green_tint: Box::new(1.into()),
            blue_tint: Box::new(2.into()),
            rotation: Box::new(3.into()),
            swap: Box::new(4.into()),
        }
    );
}

#[test]
fn test_command_ttar() {
    assert_eq!(
        parse_cmnd("TTAR 0 1 2"),
        Command::Ttar {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_brn_dmpb() {
    assert_eq!(parse_cmnd("BRN: DMPB"), Command::BrnDmpb);
}

#[test]
fn test_command_brn_dmpd() {
    assert_eq!(
        parse_cmnd("BRN: DMPD 0 1"),
        Command::BrnDmpd {
            tract_number: Box::new(0.into()),
            dendrite_number: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_brn_dmpl() {
    assert_eq!(
        parse_cmnd("BRN: DMPL 0"),
        Command::BrnDmpl {
            lobe_number: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_brn_dmpn() {
    assert_eq!(
        parse_cmnd("BRN: DMPN 0 1"),
        Command::BrnDmpn {
            lobe_number: Box::new(0.into()),
            neuron_number: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_brn_dmpt() {
    assert_eq!(
        parse_cmnd("BRN: DMPT 0"),
        Command::BrnDmpt {
            tract_number: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_brn_setd() {
    assert_eq!(
        parse_cmnd("BRN: SETD 0 1 2 3.0"),
        Command::BrnSetd {
            tract_number: Box::new(0.into()),
            dendrite_number: Box::new(1.into()),
            weight_number: Box::new(2.into()),
            new_value: Box::new(3.0f32.into()),
        }
    );
}

#[test]
fn test_command_brn_setl() {
    assert_eq!(
        parse_cmnd("BRN: SETL 0 1 3.0"),
        Command::BrnSetl {
            lobe_number: Box::new(0.into()),
            line_number: Box::new(1.into()),
            new_value: Box::new(3.0.into()),
        }
    );
}

#[test]
fn test_command_brn_setn() {
    assert_eq!(
        parse_cmnd("BRN: SETN 0 1 2 3.0"),
        Command::BrnSetn {
            lobe_number: Box::new(0.into()),
            neuron_number: Box::new(1.into()),
            state_number: Box::new(2.into()),
            new_value: Box::new(3.0.into()),
        }
    );
}

#[test]
fn test_command_brn_sett() {
    assert_eq!(
        parse_cmnd("BRN: SETT 0 1 3.0"),
        Command::BrnSett {
            tract_number: Box::new(0.into()),
            line_number: Box::new(1.into()),
            new_value: Box::new(3.0.into()),
        }
    );
}

#[test]
fn test_command_bkgd() {
    assert_eq!(
        parse_cmnd("BKGD 0 HAND 1"),
        Command::Bkgd {
            metaroom_id: Box::new(0.into()),
            background: Box::new(SString::Hand.into()),
            transition: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_brmi() {
    assert_eq!(
        parse_cmnd("BRMI 0 1"),
        Command::Brmi {
            mearoom_base: Box::new(0.into()),
            room_base: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_cmra() {
    assert_eq!(
        parse_cmnd("CMRA 0 1 2"),
        Command::Cmra {
            x: Box::new(0.into()),
            y: Box::new(1.into()),
            pan: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_cmrp() {
    assert_eq!(
        parse_cmnd("CMRP 0 1 2"),
        Command::Cmrp {
            x: Box::new(0.into()),
            y: Box::new(1.into()),
            pan: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_cmrt() {
    assert_eq!(
        parse_cmnd("CMRT 0"),
        Command::Cmrt {
            pan: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_frsh() {
    assert_eq!(parse_cmnd("FRSH"), Command::Frsh);
}

#[test]
fn test_command_line() {
    assert_eq!(
        parse_cmnd("LINE 1 2 3 4 5 6 7 8 9"),
        Command::Line {
            x1: Box::new(1.into()),
            y1: Box::new(2.into()),
            x2: Box::new(3.into()),
            y2: Box::new(4.into()),
            r: Box::new(5.into()),
            g: Box::new(6.into()),
            b: Box::new(7.into()),
            stipple_on: Box::new(8.into()),
            stipple_off: Box::new(9.into()),
        }
    );
}

#[test]
fn test_command_meta() {
    assert_eq!(
        parse_cmnd("META 1 2 3 4"),
        Command::Meta {
            metaroom_id: Box::new(1.into()),
            camera_x: Box::new(2.into()),
            camera_y: Box::new(3.into()),
            transition: Box::new(4.into()),
        }
    );
}

#[test]
fn test_command_scam() {
    assert_eq!(
        parse_cmnd("SCAM NULL 1"),
        Command::Scam {
            compound_agent: Box::new(Agent::Null.into()),
            part_number: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_snap() {
    assert_eq!(
        parse_cmnd("SNAP HAND 1 2 3 4 5"),
        Command::Snap {
            filename: Box::new(SString::Hand.into()),
            x_centre: Box::new(1.into()),
            y_centre: Box::new(2.into()),
            width: Box::new(3.into()),
            height: Box::new(4.into()),
            zoom_factor: Box::new(5.into()),
        }
    );
}

#[test]
fn test_command_trck() {
    assert_eq!(
        parse_cmnd("TRCK NULL 1 2 3 4"),
        Command::Trck {
            agent: Box::new(Agent::Null.into()),
            x_percent: Box::new(1.into()),
            y_percent: Box::new(2.into()),
            style: Box::new(3.into()),
            transition: Box::new(4.into()),
        }
    );
}

#[test]
fn test_command_wdow() {
    assert_eq!(parse_cmnd("WDOW"), Command::Wdow);
}

#[test]
fn test_command_zoom() {
    assert_eq!(
        parse_cmnd("ZOOM 1 2 3"),
        Command::Zoom {
            pixels: Box::new(1.into()),
            x: Box::new(2.into()),
            y: Box::new(3.into()),
        }
    );
}

#[test]
fn test_command_fcus() {
    assert_eq!(parse_cmnd("FCUS"), Command::Fcus);
}

#[test]
fn test_command_frmt() {
    assert_eq!(
        parse_cmnd("FRMT 1 2 3 4 5 6 7"),
        Command::Frmt {
            left_margin: Box::new(1.into()),
            top_margin: Box::new(2.into()),
            right_margin: Box::new(3.into()),
            bottom_margin: Box::new(4.into()),
            line_spacing: Box::new(5.into()),
            character_spacing: Box::new(6.into()),
            justification: Box::new(7.into()),
        }
    );
}

#[test]
fn test_command_grpl() {
    assert_eq!(
        parse_cmnd("GRPL 1 2 3 4 5"),
        Command::Grpl {
            red: Box::new(1.into()),
            green: Box::new(2.into()),
            blue: Box::new(3.into()),
            min_y: Box::new(4.into()),
            max_y: Box::new(5.into()),
        }
    );
}

#[test]
fn test_command_grpv() {
    assert_eq!(
        parse_cmnd("GRPV 1 2"),
        Command::Grpv {
            line_index: Box::new(1.into()),
            value: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_new_comp() {
    assert_eq!(
        parse_cmnd("NEW: COMP 1 2 3 HAND 4 5 6"),
        Command::NewComp {
            family: Box::new(1.into()),
            genus: Box::new(2.into()),
            species: Box::new(3.into()),
            sprite_file: Box::new(SString::Hand.into()),
            image_count: Box::new(4.into()),
            first_image: Box::new(5.into()),
            plane: Box::new(6.into()),
        }
    );
}

#[test]
fn test_command_page() {
    assert_eq!(
        parse_cmnd("PAGE 1"),
        Command::Page {
            page: Box::new(1.into())
        },
    );
}

#[test]
fn test_command_part() {
    assert_eq!(
        parse_cmnd("PART 1"),
        Command::Part {
            part_id: Box::new(1.into())
        },
    );
}

#[test]
fn test_command_pat_butt() {
    assert_eq!(
        parse_cmnd("PAT: BUTT 1 HAND 2 3 4 5 6 [] 7 8"),
        Command::PatButt {
            part_id: Box::new(1.into()),
            sprite_file: Box::new(SString::Hand.into()),
            first_image: Box::new(2.into()),
            image_count: Box::new(3.into()),
            rel_x: Box::new(4.into()),
            rel_y: Box::new(5.into()),
            rel_plane: Box::new(6.into()),
            anim_hover: Box::new(vec![].into()),
            message_id: Box::new(7.into()),
            option: Box::new(8.into()),
        }
    );
}

#[test]
fn test_command_pat_cmra() {
    assert_eq!(
        parse_cmnd("PAT: CMRA 1 HAND 2 3 4 5 6 7 8 9"),
        Command::PatCmra {
            part_id: Box::new(1.into()),
            overlay_sprite: Box::new(SString::Hand.into()),
            base_image: Box::new(2.into()),
            rel_x: Box::new(3.into()),
            rel_y: Box::new(4.into()),
            rel_plane: Box::new(5.into()),
            view_width: Box::new(6.into()),
            view_height: Box::new(7.into()),
            camera_width: Box::new(8.into()),
            camera_height: Box::new(9.into()),
        }
    );
}

#[test]
fn test_command_pat_dull() {
    assert_eq!(
        parse_cmnd("PAT: DULL 1 HAND 3 4 5 6"),
        Command::PatDull {
            part_id: Box::new(1.into()),
            sprite_file: Box::new(SString::Hand.into()),
            first_image: Box::new(3.into()),
            rel_x: Box::new(4.into()),
            rel_y: Box::new(5.into()),
            rel_plane: Box::new(6.into()),
        }
    );
}

#[test]
fn test_command_pat_fixd() {
    assert_eq!(
        parse_cmnd("PAT: FIXD 1 HAND 2 3 4 5 VOIS"),
        Command::PatFixd {
            part_id: Box::new(1.into()),
            sprite_file: Box::new(SString::Hand.into()),
            first_image: Box::new(2.into()),
            rel_x: Box::new(3.into()),
            rel_y: Box::new(4.into()),
            rel_plane: Box::new(5.into()),
            font_sprite: Box::new(SString::Vois.into()),
        }
    );
}

#[test]
fn test_command_pat_grph() {
    assert_eq!(
        parse_cmnd("PAT: GRPH 1 HAND 3 4 5 6 7"),
        Command::PatGrph {
            part_id: Box::new(1.into()),
            overlay_sprite: Box::new(SString::Hand.into()),
            base_image: Box::new(3.into()),
            rel_x: Box::new(4.into()),
            rel_y: Box::new(5.into()),
            rel_plane: Box::new(6.into()),
            num_values: Box::new(7.into()),
        }
    );
}

#[test]
fn test_command_pat_kill() {
    assert_eq!(
        parse_cmnd("PAT: KILL 1"),
        Command::PatKill {
            part_id: Box::new(1.into())
        },
    );
}

#[test]
fn test_command_pat_text() {
    assert_eq!(
        parse_cmnd("PAT: TEXT 1 HAND 2 3 4 5 6 VOIS"),
        Command::PatText {
            part_id: Box::new(1.into()),
            sprite_file: Box::new(SString::Hand.into()),
            first_image: Box::new(2.into()),
            rel_x: Box::new(3.into()),
            rel_y: Box::new(4.into()),
            rel_plane: Box::new(5.into()),
            message_id: Box::new(6.into()),
            font_sprite: Box::new(SString::Vois.into()),
        }
    );
}

#[test]
fn test_command_ptxt() {
    assert_eq!(
        parse_cmnd("PTXT VOIS"),
        Command::Ptxt {
            text: Box::new(SString::Vois.into()),
        }
    );
}

#[test]
fn test_command_ages() {
    assert_eq!(
        parse_cmnd("AGES 0"),
        Command::Ages {
            times: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_appr() {
    assert_eq!(parse_cmnd("APPR"), Command::Appr);
}

#[test]
fn test_command_aslp() {
    assert_eq!(
        parse_cmnd("ASLP 0"),
        Command::Aslp {
            asleep: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_body() {
    assert_eq!(
        parse_cmnd("BODY 0 1"),
        Command::Body {
            set_number: Box::new(0.into()),
            layer: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_born() {
    assert_eq!(parse_cmnd("BORN"), Command::Born);
}

#[test]
fn test_command_chem() {
    assert_eq!(
        parse_cmnd("CHEM 0 1"),
        Command::Chem {
            chemical: Box::new(0.into()),
            adjustment: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_dead() {
    assert_eq!(parse_cmnd("DEAD"), Command::Dead);
}

#[test]
fn test_command_dirn() {
    assert_eq!(
        parse_cmnd("DIRN 0"),
        Command::Dirn {
            direction: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_done() {
    assert_eq!(parse_cmnd("DONE"), Command::Done);
}

#[test]
fn test_command_drea() {
    assert_eq!(
        parse_cmnd("DREA 0"),
        Command::Drea {
            dream: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_driv() {
    assert_eq!(
        parse_cmnd("DRIV 0 1"),
        Command::Driv {
            drive: Box::new(0.into()),
            adjustment: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_face() {
    assert_eq!(
        parse_cmnd("FACE 0"),
        Command::Face {
            set_number: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_forf() {
    assert_eq!(
        parse_cmnd("FORF NULL"),
        Command::Forf {
            creature_to_learn_about: Box::new(Agent::Null.into()),
        }
    );
}

#[test]
fn test_command_hair() {
    assert_eq!(
        parse_cmnd("HAIR 0"),
        Command::Hair {
            stage: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_injr() {
    assert_eq!(
        parse_cmnd("INJR 0 1"),
        Command::Injr {
            organ: Box::new(0.into()),
            amount: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_like() {
    assert_eq!(
        parse_cmnd("LIKE NULL"),
        Command::Like {
            creature_state_opinion_about: Box::new(Agent::Null.into()),
        }
    );
}

#[test]
fn test_command_loci() {
    assert_eq!(
        parse_cmnd("LOCI 1 2 3 4 5"),
        Command::Loci {
            r#type: Box::new(1.into()),
            organ: Box::new(2.into()),
            tissue: Box::new(3.into()),
            id: Box::new(4.into()),
            new_value: Box::new(5.into()),
        }
    );
}

#[test]
fn test_command_ltcy() {
    assert_eq!(
        parse_cmnd("LTCY 1 2 3"),
        Command::Ltcy {
            action: Box::new(1.into()),
            min: Box::new(2.into()),
            max: Box::new(3.into()),
        }
    );
}

#[test]
fn test_command_mate() {
    assert_eq!(parse_cmnd("MATE"), Command::Mate);
}

#[test]
fn test_command_mvft() {
    assert_eq!(
        parse_cmnd("MVFT 1 2"),
        Command::Mvft {
            x: Box::new(1.into()),
            y: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_new_crea() {
    assert_eq!(
        parse_cmnd("NEW: CREA 1 NULL 2 3 4"),
        Command::NewCrea {
            family: Box::new(1.into()),
            gene_agent: Box::new(Agent::Null.into()),
            gene_slot: Box::new(2.into()),
            sex: Box::new(3.into()),
            variant: Box::new(4.into()),
        }
    );
}

#[test]
fn test_command_newc() {
    assert_eq!(
        parse_cmnd("NEWC 1 NULL 2 3 4"),
        Command::Newc {
            family: Box::new(1.into()),
            gene_agent: Box::new(Agent::Null.into()),
            gene_slot: Box::new(2.into()),
            sex: Box::new(3.into()),
            variant: Box::new(4.into()),
        }
    );
}

#[test]
fn test_command_norn() {
    assert_eq!(
        parse_cmnd("NORN NULL"),
        Command::Norn {
            creature: Box::new(Agent::Null.into()),
        }
    );
}

#[test]
fn test_command_nude() {
    assert_eq!(parse_cmnd("NUDE"), Command::Nude);
}

#[test]
fn test_command_ordr_shou() {
    assert_eq!(
        parse_cmnd("ORDR SHOU HAND"),
        Command::OrdrShou {
            speech: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_ordr_sign() {
    assert_eq!(
        parse_cmnd("ORDR SIGN HAND"),
        Command::OrdrSign {
            speech: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_ordr_writ() {
    assert_eq!(
        parse_cmnd("ORDR WRIT NULL HAND"),
        Command::OrdrWrit {
            creature: Box::new(Agent::Null.into()),
            speech: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_sayn() {
    assert_eq!(parse_cmnd("SAYN"), Command::Sayn);
}

#[test]
fn test_command_spnl() {
    assert_eq!(
        parse_cmnd("SPNL HAND 0 1"),
        Command::Spnl {
            lobe_monkier: Box::new(SString::Hand.into()),
            neuron_id: Box::new(0.into()),
            value: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_stim_shou() {
    assert_eq!(
        parse_cmnd("STIM SHOU 0 1"),
        Command::StimShou {
            stimulus: Box::new(0.into()),
            strength: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_stim_sign() {
    assert_eq!(
        parse_cmnd("STIM SIGN 0 1"),
        Command::StimSign {
            stimulus: Box::new(0.into()),
            strength: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_stim_tact() {
    assert_eq!(
        parse_cmnd("STIM TACT 0 1"),
        Command::StimTact {
            stimulus: Box::new(0.into()),
            strength: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_stim_writ() {
    assert_eq!(
        parse_cmnd("STIM WRIT NULL 0 1"),
        Command::StimWrit {
            creature: Box::new(Agent::Null.into()),
            stimulus: Box::new(0.into()),
            strength: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_sway_shou() {
    assert_eq!(
        parse_cmnd("SWAY SHOU 0 1 2 3 4 5 6 7"),
        Command::SwayShou {
            drive1: Box::new(0.into()),
            adjust1: Box::new(1.into()),
            drive2: Box::new(2.into()),
            adjust2: Box::new(3.into()),
            drive3: Box::new(4.into()),
            adjust3: Box::new(5.into()),
            drive4: Box::new(6.into()),
            adjust4: Box::new(7.into()),
        }
    );
}

#[test]
fn test_command_sway_sign() {
    assert_eq!(
        parse_cmnd("SWAY SIGN 0 1 2 3 4 5 6 7"),
        Command::SwaySign {
            drive1: Box::new(0.into()),
            adjust1: Box::new(1.into()),
            drive2: Box::new(2.into()),
            adjust2: Box::new(3.into()),
            drive3: Box::new(4.into()),
            adjust3: Box::new(5.into()),
            drive4: Box::new(6.into()),
            adjust4: Box::new(7.into()),
        }
    );
}

#[test]
fn test_command_sway_tact() {
    assert_eq!(
        parse_cmnd("SWAY TACT 0 1 2 3 4 5 6 7"),
        Command::SwayTact {
            drive1: Box::new(0.into()),
            adjust1: Box::new(1.into()),
            drive2: Box::new(2.into()),
            adjust2: Box::new(3.into()),
            drive3: Box::new(4.into()),
            adjust3: Box::new(5.into()),
            drive4: Box::new(6.into()),
            adjust4: Box::new(7.into()),
        }
    );
}

#[test]
fn test_command_sway_writ() {
    assert_eq!(
        parse_cmnd("SWAY WRIT NULL 0 1 2 3 4 5 6 7"),
        Command::SwayWrit {
            creature: Box::new(Agent::Null.into()),
            drive1: Box::new(0.into()),
            adjust1: Box::new(1.into()),
            drive2: Box::new(2.into()),
            adjust2: Box::new(3.into()),
            drive3: Box::new(4.into()),
            adjust3: Box::new(5.into()),
            drive4: Box::new(6.into()),
            adjust4: Box::new(7.into()),
        }
    );
}

#[test]
fn test_command_touc() {
    assert_eq!(parse_cmnd("TOUC"), Command::Touc);
}

#[test]
fn test_command_uncs() {
    assert_eq!(
        parse_cmnd("UNCS 0"),
        Command::Uncs {
            unconscious: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_urge_shou() {
    assert_eq!(
        parse_cmnd("URGE SHOU 0 1 2"),
        Command::UrgeShou {
            noun_stim: Box::new(0.into()),
            verb_id: Box::new(1.into()),
            verb_stim: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_urge_sign() {
    assert_eq!(
        parse_cmnd("URGE SIGN 0 1 2"),
        Command::UrgeSign {
            noun_stim: Box::new(0.into()),
            verb_id: Box::new(1.into()),
            verb_stim: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_urge_tact() {
    assert_eq!(
        parse_cmnd("URGE TACT 0 1 2"),
        Command::UrgeTact {
            noun_stim: Box::new(0.into()),
            verb_id: Box::new(1.into()),
            verb_stim: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_urge_writ() {
    assert_eq!(
        parse_cmnd("URGE WRIT NULL 3 0 1 2"),
        Command::UrgeWrit {
            creature: Box::new(Agent::Null.into()),
            noun_id: Box::new(3.into()),
            noun_stim: Box::new(0.into()),
            verb_id: Box::new(1.into()),
            verb_stim: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_vocb() {
    assert_eq!(parse_cmnd("VOCB"), Command::Vocb);
}

#[test]
fn test_command_walk() {
    assert_eq!(parse_cmnd("WALK"), Command::Walk);
}

#[test]
fn test_command_wear() {
    assert_eq!(
        parse_cmnd("WEAR 0 1 2"),
        Command::Wear {
            body_id: Box::new(0.into()),
            set_number: Box::new(1.into()),
            layer: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_zomb() {
    assert_eq!(
        parse_cmnd("ZOMB 0"),
        Command::Zomb {
            zombie: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_apro() {
    assert_eq!(
        parse_cmnd("APRO HAND"),
        Command::Apro {
            search_text: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_dbg_asrt() {
    assert_eq!(
        parse_cmnd("DBG: ASRT 1<>2"),
        Command::DbgAsrt {
            condition: Condition::Simple {
                cond_type: crate::ast::ConditionType::Ne,
                lhs: 1.into(),
                rhs: 2.into()
            }
        }
    );
}

#[test]
fn test_command_dbg_cpro() {
    assert_eq!(parse_cmnd("DBG: CPRO"), Command::DbgCpro);
}

#[test]
fn test_command_dbg_flsh() {
    assert_eq!(parse_cmnd("DBG: FLSH"), Command::DbgFlsh);
}

#[test]
fn test_command_dbg_html() {
    assert_eq!(
        parse_cmnd("DBG: HTML 0"),
        Command::DbgHtml {
            sort_order: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_dbg_outs() {
    assert_eq!(
        parse_cmnd("DBG: OUTS HAND"),
        Command::DbgOuts {
            value: Box::new(SString::Hand.into())
        }
    );
}

#[test]
fn test_command_dbg_outv() {
    assert_eq!(
        parse_cmnd("DBG: OUTV 0"),
        Command::DbgOutv {
            value: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_dbg_paws() {
    assert_eq!(parse_cmnd("DBG: PAWS"), Command::DbgPaws);
}

#[test]
fn test_command_dbg_play() {
    assert_eq!(parse_cmnd("DBG: PLAY"), Command::DbgPlay);
}

#[test]
fn test_command_dbg_poll() {
    assert_eq!(parse_cmnd("DBG: POLL"), Command::DbgPoll);
}

#[test]
fn test_command_dbg_prof() {
    assert_eq!(parse_cmnd("DBG: PROF"), Command::DbgProf);
}

#[test]
fn test_command_dbg_tack() {
    assert_eq!(
        parse_cmnd("DBG: TACK NULL"),
        Command::DbgTack {
            follow: Box::new(Agent::Null.into()),
        }
    );
}

#[test]
fn test_command_dbg_tock() {
    assert_eq!(parse_cmnd("DBG: TOCK"), Command::DbgTock);
}

#[test]
fn test_command_dbg_wtik() {
    assert_eq!(
        parse_cmnd("DBG: WTIK 0"),
        Command::DbgWtik {
            new_world_tick: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_help() {
    assert_eq!(parse_cmnd("HELP"), Command::Help);
}

#[test]
fn test_command_mann() {
    assert_eq!(
        parse_cmnd("MANN HAND"),
        Command::Mann {
            command: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_memx() {
    assert_eq!(parse_cmnd("MEMX"), Command::Memx);
}

#[test]
fn test_command_file_glob() {
    assert_eq!(
        parse_cmnd("FILE GLOB 0 HAND"),
        Command::FileGlob {
            directory: Box::new(0.into()),
            file_spec: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_file_iclo() {
    assert_eq!(parse_cmnd("FILE ICLO"), Command::FileIclo);
}

#[test]
fn test_command_file_iope() {
    assert_eq!(
        parse_cmnd("FILE IOPE 0 HAND"),
        Command::FileIope {
            directory: Box::new(0.into()),
            filename: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_file_jdel() {
    assert_eq!(
        parse_cmnd("FILE JDEL 0 HAND"),
        Command::FileJdel {
            directory: Box::new(0.into()),
            filename: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_file_oclo() {
    assert_eq!(parse_cmnd("FILE OCLO"), Command::FileOclo);
}

#[test]
fn test_command_file_oflu() {
    assert_eq!(parse_cmnd("FILE OFLU"), Command::FileOflu);
}

#[test]
fn test_command_file_oope() {
    assert_eq!(
        parse_cmnd("FILE OOPE 0 HAND 1"),
        Command::FileOope {
            directory: Box::new(0.into()),
            filename: Box::new(SString::Hand.into()),
            append: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_outs() {
    assert_eq!(
        parse_cmnd("OUTS HAND"),
        Command::Outs {
            text: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_outv() {
    assert_eq!(
        parse_cmnd("OUTV 0"),
        Command::Outv {
            value: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_outx() {
    assert_eq!(
        parse_cmnd("OUTX HAND"),
        Command::Outx {
            text: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_goto() {
    assert_eq!(
        parse_cmnd("GOTO FOOBAR"),
        Command::Goto {
            destination: String::from("FOOBAR").into()
        }
    );
}

#[test]
fn test_command_gsub() {
    assert_eq!(
        parse_cmnd("GSUB FOOBAR"),
        Command::Gsub {
            destination: String::from("FOOBAR").into()
        }
    );
}

#[test]
fn test_command_gene_clon() {
    assert_eq!(
        parse_cmnd("GENE CLON NULL 0 NORN 1"),
        Command::GeneClon {
            dest_agent: Box::new(Agent::Null.into()),
            dest_slot: Box::new(0.into()),
            source_agent: Box::new(Agent::Norn.into()),
            source_slot: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_gene_cros() {
    assert_eq!(
        parse_cmnd("GENE CROS NULL 0 NORN 1 FROM 2 3 4 5 6"),
        Command::GeneCros {
            child_agent: Box::new(Agent::Null.into()),
            child_slot: Box::new(0.into()),
            mum_agent: Box::new(Agent::Norn.into()),
            mum_slot: Box::new(1.into()),
            dad_agent: Box::new(Agent::From.into()),
            dad_slot: Box::new(2.into()),
            mum_chance_of_mutation: Box::new(3.into()),
            mum_degree_of_mutation: Box::new(4.into()),
            dad_chance_of_mutation: Box::new(5.into()),
            dad_degree_of_mutation: Box::new(6.into()),
        }
    );
}

#[test]
fn test_command_gene_kill() {
    assert_eq!(
        parse_cmnd("GENE KILL NULL 0"),
        Command::GeneKill {
            agent: Box::new(Agent::Null.into()),
            slot: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_gene_load() {
    assert_eq!(
        parse_cmnd("GENE LOAD NULL 0 HAND"),
        Command::GeneLoad {
            agent: Box::new(Agent::Null.into()),
            slot: Box::new(0.into()),
            gene_file: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_gene_move() {
    assert_eq!(
        parse_cmnd("GENE MOVE NULL 0 NORN 1"),
        Command::GeneMove {
            dest_agent: Box::new(Agent::Null.into()),
            dest_slot: Box::new(0.into()),
            source_agent: Box::new(Agent::Norn.into()),
            source_slot: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_hist_evnt() {
    assert_eq!(
        parse_cmnd("HIST EVNT HAND 0 VOIS EMID"),
        Command::HistEvnt {
            moniker: Box::new(SString::Hand.into()),
            event_type: Box::new(0.into()),
            related_moniker_1: Box::new(SString::Vois.into()),
            related_moniker_2: Box::new(SString::Emid.into()),
        }
    );
}

#[test]
fn test_command_hist_foto() {
    assert_eq!(
        parse_cmnd("HIST FOTO HAND 0 VOIS"),
        Command::HistFoto {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(0.into()),
            new_value: Box::new(SString::Vois.into()),
        }
    );
}

#[test]
fn test_command_hist_name() {
    assert_eq!(
        parse_cmnd("HIST NAME HAND VOIS"),
        Command::HistName {
            moniker: Box::new(SString::Hand.into()),
            new_name: Box::new(SString::Vois.into()),
        }
    );
}

#[test]
fn test_command_hist_utxt() {
    assert_eq!(
        parse_cmnd("HIST UTXT HAND 0 VOIS"),
        Command::HistUtxt {
            moniker: Box::new(SString::Hand.into()),
            event_no: Box::new(0.into()),
            new_value: Box::new(SString::Vois.into()),
        }
    );
}

#[test]
fn test_command_hist_wipe() {
    assert_eq!(
        parse_cmnd("HIST WIPE VOIS"),
        Command::HistWipe {
            moniker: Box::new(SString::Vois.into()),
        }
    );
}

#[test]
fn test_command_clac() {
    assert_eq!(
        parse_cmnd("CLAC 0"),
        Command::Clac {
            message: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_clik() {
    assert_eq!(
        parse_cmnd("CLIK 0 1 2"),
        Command::Clik {
            message_1: Box::new(0.into()),
            message_2: Box::new(1.into()),
            message_3: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_imsk() {
    assert_eq!(
        parse_cmnd("IMSK 0"),
        Command::Imsk {
            mask: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_mous() {
    assert_eq!(
        parse_cmnd("MOUS 0"),
        Command::Mous {
            behaviour: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_pure() {
    assert_eq!(
        parse_cmnd("PURE 0"),
        Command::Pure {
            value: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_tran() {
    assert_eq!(
        parse_cmnd("TRAN 0 1"),
        Command::Tran {
            transparency: Box::new(0.into()),
            part_no: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_addb() {
    assert_eq!(
        parse_cmnd("ADDB 0 HAND"),
        Command::Addb {
            metaroom_id: Box::new(0.into()),
            background_file: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_altr() {
    assert_eq!(
        parse_cmnd("ALTR 0 1 2"),
        Command::Altr {
            room_id: Box::new(0.into()),
            ca_index: Box::new(1.into()),
            ca_delta: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_cacl() {
    assert_eq!(
        parse_cmnd("CACL 0 1 2 3"),
        Command::Cacl {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            ca_index: Box::new(3.into()),
        }
    );
}

#[test]
fn test_command_calc() {
    assert_eq!(parse_cmnd("CALC"), Command::Calc);
}

#[test]
fn test_command_delm() {
    assert_eq!(
        parse_cmnd("DELM 0"),
        Command::Delm {
            metaroom_id: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_delr() {
    assert_eq!(
        parse_cmnd("DELR 0"),
        Command::Delr {
            room_id: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_dmap() {
    assert_eq!(
        parse_cmnd("DMAP 0"),
        Command::Dmap {
            debug_map: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_doca() {
    assert_eq!(
        parse_cmnd("DOCA 0"),
        Command::Doca {
            no_of_updates: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_door() {
    assert_eq!(
        parse_cmnd("DOOR 0 1 2"),
        Command::Door {
            room_id1: Box::new(0.into()),
            room_id2: Box::new(1.into()),
            permiability: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_emit() {
    assert_eq!(
        parse_cmnd("EMIT 0 1"),
        Command::Emit {
            ca_index: Box::new(0.into()),
            amount: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_link() {
    assert_eq!(
        parse_cmnd("LINK 0 1 2"),
        Command::Link {
            room1: Box::new(0.into()),
            room2: Box::new(1.into()),
            permiability: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_mapd() {
    assert_eq!(
        parse_cmnd("MAPD 0 1"),
        Command::Mapd {
            width: Box::new(0.into()),
            height: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_mapk() {
    assert_eq!(parse_cmnd("MAPK"), Command::Mapk);
}

#[test]
fn test_command_perm() {
    assert_eq!(
        parse_cmnd("PERM 0"),
        Command::Perm {
            permiability: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_prop() {
    assert_eq!(
        parse_cmnd("PROP 0 1 2"),
        Command::Prop {
            room_id: Box::new(0.into()),
            ca_index: Box::new(1.into()),
            ca_value: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_rate() {
    assert_eq!(
        parse_cmnd("RATE 0 1 2 3 4"),
        Command::Rate {
            room_type: Box::new(0.into()),
            ca_index: Box::new(1.into()),
            gain: Box::new(2.into()),
            loss: Box::new(3.into()),
            diffusion: Box::new(4.into()),
        }
    );
}

#[test]
fn test_command_rtyp() {
    assert_eq!(
        parse_cmnd("RTYP 0 1"),
        Command::Rtyp {
            room_id: Box::new(0.into()),
            room_type: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_accg() {
    assert_eq!(
        parse_cmnd("ACCG 0"),
        Command::Accg {
            acceleration: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_aero() {
    assert_eq!(
        parse_cmnd("AERO 0"),
        Command::Aero {
            aerodynamics: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_elas() {
    assert_eq!(
        parse_cmnd("ELAS 0"),
        Command::Elas {
            elasticity: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_flto() {
    assert_eq!(
        parse_cmnd("FLTO 0 1"),
        Command::Flto {
            screen_x: Box::new(0.into()),
            screen_y: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_frel() {
    assert_eq!(
        parse_cmnd("FREL NULL"),
        Command::Frel {
            relative: Box::new(Agent::Null.into()),
        }
    );
}

#[test]
fn test_command_fric() {
    assert_eq!(
        parse_cmnd("FRIC 0"),
        Command::Fric {
            friction: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_mvby() {
    assert_eq!(
        parse_cmnd("MVBY 0 1"),
        Command::Mvby {
            delta_x: Box::new(0.into()),
            delta_y: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_mvsf() {
    assert_eq!(
        parse_cmnd("MVSF 0 1"),
        Command::Mvsf {
            x: Box::new(0.into()),
            y: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_mvto() {
    assert_eq!(
        parse_cmnd("MVTO 0 1"),
        Command::Mvto {
            x: Box::new(0.into()),
            y: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_velo() {
    assert_eq!(
        parse_cmnd("VELO 0 1"),
        Command::Velo {
            x_velocity: Box::new(0.into()),
            y_velocity: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_prt_bang() {
    assert_eq!(
        parse_cmnd("PRT: BANG 0"),
        Command::PrtBang {
            bang_strength: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_prt_inew() {
    assert_eq!(
        parse_cmnd("PRT: INEW 0 HAND VOIS 1 2 3"),
        Command::PrtInew {
            id: Box::new(0.into()),
            name: Box::new(SString::Hand.into()),
            description: Box::new(SString::Vois.into()),
            x: Box::new(1.into()),
            y: Box::new(2.into()),
            message_num: Box::new(3.into()),
        }
    );
}

#[test]
fn test_command_prt_izap() {
    assert_eq!(
        parse_cmnd("PRT: IZAP 0"),
        Command::PrtIzap {
            id: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_prt_join() {
    assert_eq!(
        parse_cmnd("PRT: JOIN NULL 0 NORN 2"),
        Command::PrtJoin {
            source_agent: Box::new(Agent::Null.into()),
            output_id: Box::new(0.into()),
            dest_agent: Box::new(Agent::Norn.into()),
            input_id: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_prt_krak() {
    assert_eq!(
        parse_cmnd("PRT: KRAK NULL 0 1"),
        Command::PrtKrak {
            agent: Box::new(Agent::Null.into()),
            in_or_out: Box::new(0.into()),
            port_index: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_prt_onew() {
    assert_eq!(
        parse_cmnd("PRT: ONEW 0 HAND VOIS 1 2"),
        Command::PrtOnew {
            id: Box::new(0.into()),
            name: Box::new(SString::Hand.into()),
            description: Box::new(SString::Vois.into()),
            x: Box::new(1.into()),
            y: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_prt_ozap() {
    assert_eq!(
        parse_cmnd("PRT: OZAP 0"),
        Command::PrtOzap {
            id: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_prt_send() {
    assert_eq!(
        parse_cmnd("PRT: SEND 0 1"),
        Command::PrtSend {
            id: Box::new(0.into()),
            data: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_pray_garb() {
    assert_eq!(
        parse_cmnd("PRAY GARB 0"),
        Command::PrayGarb {
            force: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_pray_refr() {
    assert_eq!(parse_cmnd("PRAY REFR"), Command::PrayRefr);
}

#[test]
fn test_command_gids_fmly() {
    assert_eq!(
        parse_cmnd("GIDS FMLY 0"),
        Command::GidsFmly {
            family: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_gids_gnus() {
    assert_eq!(
        parse_cmnd("GIDS GNUS 0 1"),
        Command::GidsGnus {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_gids_root() {
    assert_eq!(parse_cmnd("GIDS ROOT"), Command::GidsRoot);
}

#[test]
fn test_command_gids_spcs() {
    assert_eq!(
        parse_cmnd("GIDS SPCS 0 1 2"),
        Command::GidsSpcs {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_inst() {
    assert_eq!(parse_cmnd("INST"), Command::Inst);
}

#[test]
fn test_command_lock() {
    assert_eq!(parse_cmnd("LOCK"), Command::Lock);
}

#[test]
fn test_command_scrx() {
    assert_eq!(
        parse_cmnd("SCRX 0 1 2 3"),
        Command::Scrx {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            event: Box::new(3.into()),
        }
    );
}

#[test]
fn test_command_slow() {
    assert_eq!(parse_cmnd("SLOW"), Command::Slow);
}

#[test]
fn test_command_sorc() {
    assert_eq!(
        parse_cmnd("SORC 0 1 2 3"),
        Command::Sorc {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            event: Box::new(3.into()),
        }
    );
}

#[test]
fn test_command_stop() {
    assert_eq!(parse_cmnd("STOP"), Command::Stop);
}

#[test]
fn test_command_stpt() {
    assert_eq!(parse_cmnd("STPT"), Command::Stpt);
}

#[test]
fn test_command_unlk() {
    assert_eq!(parse_cmnd("UNLK"), Command::Unlk);
}

#[test]
fn test_command_wait() {
    assert_eq!(
        parse_cmnd("WAIT 0"),
        Command::Wait {
            ticks: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_fade() {
    assert_eq!(parse_cmnd("FADE"), Command::Fade);
}

#[test]
fn test_command_mclr() {
    assert_eq!(
        parse_cmnd("MCLR 0 1"),
        Command::Mclr {
            x: Box::new(0.into()),
            y: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_midi() {
    assert_eq!(
        parse_cmnd("MIDI HAND"),
        Command::Midi {
            midi_file: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_mmsc() {
    assert_eq!(
        parse_cmnd("MMSC 0 1 HAND"),
        Command::Mmsc {
            x: Box::new(0.into()),
            y: Box::new(1.into()),
            track_name: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_rclr() {
    assert_eq!(
        parse_cmnd("RCLR 0 1"),
        Command::Rclr {
            x: Box::new(0.into()),
            y: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_rmsc() {
    assert_eq!(
        parse_cmnd("RMSC 0 1 HAND"),
        Command::Rmsc {
            x: Box::new(0.into()),
            y: Box::new(1.into()),
            track_name: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_sezz() {
    assert_eq!(
        parse_cmnd("SEZZ HAND"),
        Command::Sezz {
            text: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_sndc() {
    assert_eq!(
        parse_cmnd("SNDC HAND"),
        Command::Sndc {
            sound_file: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_snde() {
    assert_eq!(
        parse_cmnd("SNDE HAND"),
        Command::Snde {
            sound_file: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_sndl() {
    assert_eq!(
        parse_cmnd("SNDL HAND"),
        Command::Sndl {
            sound_file: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_sndq() {
    assert_eq!(
        parse_cmnd("SNDQ HAND 0"),
        Command::Sndq {
            sound_file: Box::new(SString::Hand.into()),
            delay: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_stpc() {
    assert_eq!(parse_cmnd("STPC"), Command::Stpc);
}

#[test]
fn test_command_strk() {
    assert_eq!(
        parse_cmnd("STRK 0 HAND"),
        Command::Strk {
            latency: Box::new(0.into()),
            track: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_voic() {
    assert_eq!(
        parse_cmnd("VOIC 0 1 2"),
        Command::Voic {
            genus: Box::new(0.into()),
            gender: Box::new(1.into()),
            age: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_vois() {
    assert_eq!(
        parse_cmnd("VOIS HAND"),
        Command::Vois {
            voice_name: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_volm() {
    assert_eq!(
        parse_cmnd("VOLM 0"),
        Command::Volm {
            volume: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_wpau() {
    assert_eq!(
        parse_cmnd("WPAU 0"),
        Command::Wpau {
            paused: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_absv() {
    assert_eq!(
        parse_cmnd("ABSV VELX"),
        Command::Absv {
            var: Box::new(Variable::Velx),
        }
    );
}

#[test]
fn test_command_adds() {
    assert_eq!(
        parse_cmnd("ADDS VELX HAND"),
        Command::Adds {
            var: Box::new(Variable::Velx),
            append: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_addv() {
    assert_eq!(
        parse_cmnd("ADDV VELX 0"),
        Command::Addv {
            var: Box::new(Variable::Velx),
            sum: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_andv() {
    assert_eq!(
        parse_cmnd("ANDV VELX 0"),
        Command::Andv {
            var: Box::new(Variable::Velx),
            value: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_char() {
    assert_eq!(
        parse_cmnd("CHAR VELX 0 1"),
        Command::Char {
            string: Box::new(Variable::Velx),
            index: Box::new(0.into()),
            character: Box::new(1.into()),
        }
    );
}

#[test]
fn test_command_delg() {
    assert_eq!(
        parse_cmnd("DELG HAND"),
        Command::Delg {
            variable_name: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_divv() {
    assert_eq!(
        parse_cmnd("DIVV VELX 0"),
        Command::Divv {
            var: Box::new(Variable::Velx),
            div: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_modv() {
    assert_eq!(
        parse_cmnd("MODV VELX 0"),
        Command::Modv {
            var: Box::new(Variable::Velx),
            r#mod: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_mulv() {
    assert_eq!(
        parse_cmnd("MULV VELX 0"),
        Command::Mulv {
            var: Box::new(Variable::Velx),
            mul: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_negv() {
    assert_eq!(
        parse_cmnd("NEGV VELX"),
        Command::Negv {
            var: Box::new(Variable::Velx),
        }
    );
}

#[test]
fn test_command_orrv() {
    assert_eq!(
        parse_cmnd("ORRV VELX 0"),
        Command::Orrv {
            var: Box::new(Variable::Velx),
            value: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_reaf() {
    assert_eq!(parse_cmnd("REAF"), Command::Reaf);
}

#[test]
fn test_command_seta() {
    assert_eq!(
        parse_cmnd("SETA VELX NULL"),
        Command::Seta {
            var: Box::new(Variable::Velx),
            value: Box::new(Agent::Null.into()),
        }
    );
}

#[test]
fn test_command_sets() {
    assert_eq!(
        parse_cmnd("SETS VELX HAND"),
        Command::Sets {
            var: Box::new(Variable::Velx),
            value: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_setv() {
    assert_eq!(
        parse_cmnd("SETV VELX 0"),
        Command::Setv {
            var: Box::new(Variable::Velx),
            value: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_subv() {
    assert_eq!(
        parse_cmnd("SUBV VELX 0"),
        Command::Subv {
            var: Box::new(Variable::Velx),
            sub: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_targ() {
    assert_eq!(
        parse_cmnd("TARG NULL"),
        Command::Targ {
            agent: Box::new(Agent::Null.into()),
        }
    );
}

#[test]
fn test_command_cabn() {
    assert_eq!(
        parse_cmnd("CABN 0 1 2 3"),
        Command::Cabn {
            left: Box::new(0.into()),
            top: Box::new(1.into()),
            right: Box::new(2.into()),
            bottom: Box::new(3.into()),
        }
    );
}

#[test]
fn test_command_cabp() {
    assert_eq!(
        parse_cmnd("CABP 0"),
        Command::Cabp {
            plane: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_cabv() {
    assert_eq!(
        parse_cmnd("CABV 0"),
        Command::Cabv {
            cabin_room_id: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_cabw() {
    assert_eq!(
        parse_cmnd("CABW 0"),
        Command::Cabw {
            cabin_capacity: Box::new(0.into()),
        }
    );
}

#[test]
fn test_command_dpas() {
    assert_eq!(
        parse_cmnd("DPAS 0 1 2"),
        Command::Dpas {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
        }
    );
}

#[test]
fn test_command_gpas() {
    assert_eq!(
        parse_cmnd("GPAS 0 1 2 3"),
        Command::Gpas {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            rect_to_use: Box::new(3.into()),
        }
    );
}

#[test]
fn test_command_new_vhcl() {
    assert_eq!(
        parse_cmnd("NEW: VHCL 0 1 2 HAND 3 4 5"),
        Command::NewVhcl {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            sprite_file: Box::new(SString::Hand.into()),
            image_count: Box::new(3.into()),
            first_image: Box::new(4.into()),
            plane: Box::new(5.into()),
        }
    );
}

#[test]
fn test_command_rpas() {
    assert_eq!(
        parse_cmnd("RPAS NULL NORN"),
        Command::Rpas {
            vehicle: Box::new(Agent::Null.into()),
            passenger: Box::new(Agent::Norn.into()),
        }
    );
}

#[test]
fn test_command_spas() {
    assert_eq!(
        parse_cmnd("SPAS NULL NORN"),
        Command::Spas {
            vehicle: Box::new(Agent::Null.into()),
            new_passenger: Box::new(Agent::Norn.into()),
        }
    );
}

#[test]
fn test_command_delw() {
    assert_eq!(
        parse_cmnd("DELW HAND"),
        Command::Delw {
            world_name: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_load() {
    assert_eq!(
        parse_cmnd("LOAD HAND"),
        Command::Load {
            world_name: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_pswd() {
    assert_eq!(
        parse_cmnd("PSWD HAND"),
        Command::Pswd {
            world_name: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_quit() {
    assert_eq!(parse_cmnd("QUIT"), Command::Quit);
}

#[test]
fn test_command_rgam() {
    assert_eq!(parse_cmnd("RGAM"), Command::Rgam);
}

#[test]
fn test_command_save() {
    assert_eq!(parse_cmnd("SAVE"), Command::Save);
}

#[test]
fn test_command_tntw() {
    assert_eq!(
        parse_cmnd("TNTW 0"),
        Command::Tntw {
            index: Box::new(0.into())
        },
    );
}

#[test]
fn test_command_wrld() {
    assert_eq!(
        parse_cmnd("WRLD HAND"),
        Command::Wrld {
            world_name: Box::new(SString::Hand.into()),
        }
    );
}

#[test]
fn test_command_wtnt() {
    assert_eq!(
        parse_cmnd("WTNT 1 2 3 4 5 6"),
        Command::Wtnt {
            index: Box::new(1.into()),
            red_tint: Box::new(2.into()),
            green_tint: Box::new(3.into()),
            blue_tint: Box::new(4.into()),
            rotation: Box::new(5.into()),
            swap: Box::new(6.into()),
        }
    );
}
