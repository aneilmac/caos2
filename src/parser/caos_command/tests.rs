use crate::{
    ast::{Agent, Condition, SString, Variable},
    parser::CaosParser,
};
use pest::Parser;

use super::*;

#[test]
fn test_command_anim() {
    for p in CaosParser::parse(Rule::command, "ANIM [1 2]").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Anim {
                pose_list: vec![1, 2].into()
            }
        );
    }
}

#[test]
fn test_command_anms() {
    for p in CaosParser::parse(Rule::command, "ANMS HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Anms {
                anim_string: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_attr() {
    for p in CaosParser::parse(Rule::command, "ATTR 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Attr {
                attributes: 0.into()
            }
        );
    }
}

#[test]
fn test_command_base() {
    for p in CaosParser::parse(Rule::command, "BASE 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Base { index: 0.into() }
        );
    }
}

#[test]
fn test_command_bbvr() {
    for p in CaosParser::parse(Rule::command, "BHVR 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Bhvr {
                permissions: 0.into()
            }
        );
    }
}

#[test]
fn test_command_frat() {
    for p in CaosParser::parse(Rule::command, "FRAT 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Frat {
                framerate: 0.into()
            }
        );
    }
}

#[test]
fn test_command_gait() {
    for p in CaosParser::parse(Rule::command, "GAIT 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Gait {
                gait_number: 0.into()
            }
        );
    }
}

#[test]
fn test_command_gall() {
    for p in CaosParser::parse(Rule::command, "GALL HAND 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Gall {
                sprite_file: SString::Hand.into(),
                first_image: 0.into()
            }
        );
    }
}

#[test]
fn test_command_hand() {
    for p in CaosParser::parse(Rule::command, "HAND HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Hand {
                name_for_the_hand: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_kill() {
    for p in CaosParser::parse(Rule::command, "KILL NULL").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Kill {
                agent: Agent::Null.into()
            }
        );
    }
}

#[test]
fn test_command_mesg_writ() {
    for p in CaosParser::parse(Rule::command, "MESG WRIT NULL 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::MesgWrit {
                agent: Agent::Null.into(),
                message_id: 0.into()
            }
        );
    }
}

#[test]
fn test_command_mesg_wrt() {
    for p in CaosParser::parse(Rule::command, "MESG WRT+ NULL 0 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::MesgWritPlus {
                agent: Agent::Null.into(),
                message_id: 0.into(),
                param_1: 1.into(),
                param_2: 2.into(),
                delay: 3.into()
            }
        );
    }
}

#[test]
fn test_command_mira() {
    for p in CaosParser::parse(Rule::command, "MIRA 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mira { on_off: 0.into() }
        );
    }
}

#[test]
fn test_command_new_simp() {
    for p in CaosParser::parse(Rule::command, "NEW: SIMP 0 1 2 HAND 3 4 5").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::NewSimp {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into(),
                sprite_file: SString::Hand.into(),
                image_count: 3.into(),
                first_image: 4.into(),
                plane: 5.into()
            }
        );
    }
}

#[test]
fn test_command_nohh() {
    for p in CaosParser::parse(Rule::command, "NOHH").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Nohh);
    }
}

#[test]
fn test_command_over() {
    for p in CaosParser::parse(Rule::command, "OVER").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Over);
    }
}

#[test]
fn test_command_paus() {
    for p in CaosParser::parse(Rule::command, "PAUS 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Paus { paused: 0.into() }
        );
    }
}

#[test]
fn test_command_plne() {
    for p in CaosParser::parse(Rule::command, "PLNE 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Plne { plane: 0.into() }
        );
    }
}

#[test]
fn test_command_pose() {
    for p in CaosParser::parse(Rule::command, "POSE 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Pose { pose: 0.into() }
        );
    }
}

#[test]
fn test_command_puhl() {
    for p in CaosParser::parse(Rule::command, "PUHL 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Puhl {
                pose: 0.into(),
                x: 1.into(),
                y: 2.into()
            }
        );
    }
}

#[test]
fn test_command_pupt() {
    for p in CaosParser::parse(Rule::command, "PUPT 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Pupt {
                pose: 0.into(),
                x: 1.into(),
                y: 2.into()
            }
        );
    }
}

#[test]
fn test_command_rnge() {
    for p in CaosParser::parse(Rule::command, "RNGE 0.0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Rnge {
                distance: 0.0f32.into()
            }
        );
    }
}

#[test]
fn test_command_rtar() {
    for p in CaosParser::parse(Rule::command, "RTAR 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Rtar {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into()
            }
        );
    }
}

#[test]
fn test_command_show() {
    for p in CaosParser::parse(Rule::command, "SHOW 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Show {
                visibility: 0.into()
            }
        );
    }
}

#[test]
fn test_command_star() {
    for p in CaosParser::parse(Rule::command, "STAR 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Star {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into()
            }
        );
    }
}

#[test]
fn test_command_tick() {
    for p in CaosParser::parse(Rule::command, "TICK 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Tick {
                tick_rate: 0.into()
            }
        );
    }
}

#[test]
fn test_command_tint() {
    for p in CaosParser::parse(Rule::command, "TINT 0 1 2 3 4").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Tint {
                red_tint: 0.into(),
                green_tint: 1.into(),
                blue_tint: 2.into(),
                rotation: 3.into(),
                swap: 4.into()
            }
        );
    }
}

#[test]
fn test_command_ttar() {
    for p in CaosParser::parse(Rule::command, "TTAR 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Ttar {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into()
            }
        );
    }
}

#[test]
fn test_command_brn_dmpb() {
    for p in CaosParser::parse(Rule::command, "BRN: DMPB").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::BrnDmpb);
    }
}

#[test]
fn test_command_brn_dmpd() {
    for p in CaosParser::parse(Rule::command, "BRN: DMPD 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::BrnDmpd {
                tract_number: 0.into(),
                dendrite_number: 1.into()
            }
        );
    }
}

#[test]
fn test_command_brn_dmpl() {
    for p in CaosParser::parse(Rule::command, "BRN: DMPL 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::BrnDmpl {
                lobe_number: 0.into()
            }
        );
    }
}

#[test]
fn test_command_brn_dmpn() {
    for p in CaosParser::parse(Rule::command, "BRN: DMPN 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::BrnDmpn {
                lobe_number: 0.into(),
                neuron_number: 1.into()
            }
        );
    }
}

#[test]
fn test_command_brn_dmpt() {
    for p in CaosParser::parse(Rule::command, "BRN: DMPT 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::BrnDmpt {
                tract_number: 0.into()
            }
        );
    }
}

#[test]
fn test_command_brn_setd() {
    for p in CaosParser::parse(Rule::command, "BRN: SETD 0 1 2 3.0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::BrnSetd {
                tract_number: 0.into(),
                dendrite_number: 1.into(),
                weight_number: 2.into(),
                new_value: 3.0f32.into()
            }
        );
    }
}

#[test]
fn test_command_brn_setl() {
    for p in CaosParser::parse(Rule::command, "BRN: SETL 0 1 3.0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::BrnSetl {
                lobe_number: 0.into(),
                line_number: 1.into(),
                new_value: 3.0.into()
            }
        );
    }
}

#[test]
fn test_command_brn_setn() {
    for p in CaosParser::parse(Rule::command, "BRN: SETN 0 1 2 3.0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::BrnSetn {
                lobe_number: 0.into(),
                neuron_number: 1.into(),
                state_number: 2.into(),
                new_value: 3.0.into()
            }
        );
    }
}

#[test]
fn test_command_brn_sett() {
    for p in CaosParser::parse(Rule::command, "BRN: SETT 0 1 3.0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::BrnSett {
                tract_number: 0.into(),
                line_number: 1.into(),
                new_value: 3.0.into()
            }
        );
    }
}

#[test]
fn test_command_bkgd() {
    for p in CaosParser::parse(Rule::command, "BKGD 0 HAND 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Bkgd {
                metaroom_id: 0.into(),
                background: SString::Hand.into(),
                transition: 1.into()
            }
        );
    }
}

#[test]
fn test_command_brmi() {
    for p in CaosParser::parse(Rule::command, "BRMI 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Brmi {
                mearoom_base: 0.into(),
                room_base: 1.into()
            }
        );
    }
}

#[test]
fn test_command_cmra() {
    for p in CaosParser::parse(Rule::command, "CMRA 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Cmra {
                x: 0.into(),
                y: 1.into(),
                pan: 2.into()
            }
        );
    }
}

#[test]
fn test_command_cmrp() {
    for p in CaosParser::parse(Rule::command, "CMRP 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Cmrp {
                x: 0.into(),
                y: 1.into(),
                pan: 2.into()
            }
        );
    }
}

#[test]
fn test_command_cmrt() {
    for p in CaosParser::parse(Rule::command, "CMRT 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Cmrt { pan: 0.into() }
        );
    }
}

#[test]
fn test_command_frsh() {
    for p in CaosParser::parse(Rule::command, "FRSH").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Frsh);
    }
}

#[test]
fn test_command_line() {
    for p in CaosParser::parse(Rule::command, "LINE 1 2 3 4 5 6 7 8 9").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Line {
                x1: 1.into(),
                y1: 2.into(),
                x2: 3.into(),
                y2: 4.into(),
                r: 5.into(),
                g: 6.into(),
                b: 7.into(),
                stipple_on: 8.into(),
                stipple_off: 9.into()
            }
        );
    }
}

#[test]
fn test_command_meta() {
    for p in CaosParser::parse(Rule::command, "META 1 2 3 4").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Meta {
                metaroom_id: 1.into(),
                camera_x: 2.into(),
                camera_y: 3.into(),
                transition: 4.into()
            }
        );
    }
}

#[test]
fn test_command_scam() {
    for p in CaosParser::parse(Rule::command, "SCAM NULL 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Scam {
                compound_agent: Agent::Null.into(),
                part_number: 1.into()
            }
        );
    }
}

#[test]
fn test_command_snap() {
    for p in CaosParser::parse(Rule::command, "SNAP HAND 1 2 3 4 5").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Snap {
                filename: SString::Hand.into(),
                x_centre: 1.into(),
                y_centre: 2.into(),
                width: 3.into(),
                height: 4.into(),
                zoom_factor: 5.into()
            }
        );
    }
}

#[test]
fn test_command_trck() {
    for p in CaosParser::parse(Rule::command, "TRCK NULL 1 2 3 4").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Trck {
                agent: Agent::Null.into(),
                x_percent: 1.into(),
                y_percent: 2.into(),
                style: 3.into(),
                transition: 4.into()
            }
        );
    }
}

#[test]
fn test_command_wdow() {
    for p in CaosParser::parse(Rule::command, "WDOW").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Wdow);
    }
}

#[test]
fn test_command_zoom() {
    for p in CaosParser::parse(Rule::command, "ZOOM 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Zoom {
                pixels: 1.into(),
                x: 2.into(),
                y: 3.into()
            }
        );
    }
}

#[test]
fn test_command_fcus() {
    for p in CaosParser::parse(Rule::command, "FCUS").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Fcus);
    }
}

#[test]
fn test_command_frmt() {
    for p in CaosParser::parse(Rule::command, "FRMT 1 2 3 4 5 6 7").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Frmt {
                left_margin: 1.into(),
                top_margin: 2.into(),
                right_margin: 3.into(),
                bottom_margin: 4.into(),
                line_spacing: 5.into(),
                character_spacing: 6.into(),
                justification: 7.into()
            }
        );
    }
}

#[test]
fn test_command_grpl() {
    for p in CaosParser::parse(Rule::command, "GRPL 1 2 3 4 5").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Grpl {
                red: 1.into(),
                green: 2.into(),
                blue: 3.into(),
                min_y: 4.into(),
                max_y: 5.into()
            }
        );
    }
}

#[test]
fn test_command_grpv() {
    for p in CaosParser::parse(Rule::command, "GRPV 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Grpv {
                line_index: 1.into(),
                value: 2.into()
            }
        );
    }
}

#[test]
fn test_command_new_comp() {
    for p in CaosParser::parse(Rule::command, "NEW: COMP 1 2 3 HAND 4 5 6").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::NewComp {
                family: 1.into(),
                genus: 2.into(),
                species: 3.into(),
                sprite_file: SString::Hand.into(),
                image_count: 4.into(),
                first_image: 5.into(),
                plane: 6.into()
            }
        );
    }
}

#[test]
fn test_command_page() {
    for p in CaosParser::parse(Rule::command, "PAGE 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Page { page: 1.into() }
        );
    }
}

#[test]
fn test_command_part() {
    for p in CaosParser::parse(Rule::command, "PART 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Part { part_id: 1.into() }
        );
    }
}

#[test]
fn test_command_pat_butt() {
    for p in CaosParser::parse(Rule::command, "PAT: BUTT 1 HAND 2 3 4 5 6 [] 7 8").expect("Parsed")
    {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PatButt {
                part_id: 1.into(),
                sprite_file: SString::Hand.into(),
                first_image: 2.into(),
                image_count: 3.into(),
                rel_x: 4.into(),
                rel_y: 5.into(),
                rel_plane: 6.into(),
                anim_hover: vec![].into(),
                message_id: 7.into(),
                option: 8.into()
            }
        );
    }
}

#[test]
fn test_command_pat_cmra() {
    for p in CaosParser::parse(Rule::command, "PAT: CMRA 1 HAND 2 3 4 5 6 7 8 9").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PatCmra {
                part_id: 1.into(),
                overlay_sprite: SString::Hand.into(),
                base_image: 2.into(),
                rel_x: 3.into(),
                rel_y: 4.into(),
                rel_plane: 5.into(),
                view_width: 6.into(),
                view_height: 7.into(),
                camera_width: 8.into(),
                camera_height: 9.into()
            }
        );
    }
}

#[test]
fn test_command_pat_dull() {
    for p in CaosParser::parse(Rule::command, "PAT: DULL 1 HAND 3 4 5 6").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PatDull {
                part_id: 1.into(),
                sprite_file: SString::Hand.into(),
                first_image: 3.into(),
                rel_x: 4.into(),
                rel_y: 5.into(),
                rel_plane: 6.into()
            }
        );
    }
}

#[test]
fn test_command_pat_fixd() {
    for p in CaosParser::parse(Rule::command, "PAT: FIXD 1 HAND 2 3 4 5 VOIS").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PatFixd {
                part_id: 1.into(),
                sprite_file: SString::Hand.into(),
                first_image: 2.into(),
                rel_x: 3.into(),
                rel_y: 4.into(),
                rel_plane: 5.into(),
                font_sprite: SString::Vois.into()
            }
        );
    }
}

#[test]
fn test_command_pat_grph() {
    for p in CaosParser::parse(Rule::command, "PAT: GRPH 1 HAND 3 4 5 6 7").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PatGrph {
                part_id: 1.into(),
                overlay_sprite: SString::Hand.into(),
                base_image: 3.into(),
                rel_x: 4.into(),
                rel_y: 5.into(),
                rel_plane: 6.into(),
                num_values: 7.into()
            }
        );
    }
}

#[test]
fn test_command_pat_kill() {
    for p in CaosParser::parse(Rule::command, "PAT: KILL 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PatKill { part_id: 1.into() }
        );
    }
}

#[test]
fn test_command_pat_text() {
    for p in CaosParser::parse(Rule::command, "PAT: TEXT 1 HAND 2 3 4 5 6 VOIS").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PatText {
                part_id: 1.into(),
                sprite_file: SString::Hand.into(),
                first_image: 2.into(),
                rel_x: 3.into(),
                rel_y: 4.into(),
                rel_plane: 5.into(),
                message_id: 6.into(),
                font_sprite: SString::Vois.into()
            }
        );
    }
}

#[test]
fn test_command_ptxt() {
    for p in CaosParser::parse(Rule::command, "PTXT VOIS").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Ptxt {
                text: SString::Vois.into()
            }
        );
    }
}

#[test]
fn test_command_ages() {
    for p in CaosParser::parse(Rule::command, "AGES 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Ages { times: 0.into() }
        );
    }
}

#[test]
fn test_command_appr() {
    for p in CaosParser::parse(Rule::command, "APPR").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Appr);
    }
}

#[test]
fn test_command_aslp() {
    for p in CaosParser::parse(Rule::command, "ASLP 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Aslp { asleep: 0.into() }
        );
    }
}

#[test]
fn test_command_body() {
    for p in CaosParser::parse(Rule::command, "BODY 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Body {
                set_number: 0.into(),
                layer: 1.into()
            }
        );
    }
}

#[test]
fn test_command_born() {
    for p in CaosParser::parse(Rule::command, "BORN").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Born);
    }
}

#[test]
fn test_command_chem() {
    for p in CaosParser::parse(Rule::command, "CHEM 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Chem {
                chemical: 0.into(),
                adjustment: 1.into()
            }
        );
    }
}

#[test]
fn test_command_dead() {
    for p in CaosParser::parse(Rule::command, "DEAD").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Dead);
    }
}

#[test]
fn test_command_dirn() {
    for p in CaosParser::parse(Rule::command, "DIRN 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Dirn {
                direction: 0.into()
            }
        );
    }
}

#[test]
fn test_command_done() {
    for p in CaosParser::parse(Rule::command, "DONE").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Done);
    }
}

#[test]
fn test_command_drea() {
    for p in CaosParser::parse(Rule::command, "DREA 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Drea { dream: 0.into() }
        );
    }
}

#[test]
fn test_command_driv() {
    for p in CaosParser::parse(Rule::command, "DRIV 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Driv {
                drive: 0.into(),
                adjustment: 1.into()
            }
        );
    }
}

#[test]
fn test_command_face() {
    for p in CaosParser::parse(Rule::command, "FACE 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Face {
                set_number: 0.into()
            }
        );
    }
}

#[test]
fn test_command_forf() {
    for p in CaosParser::parse(Rule::command, "FORF NULL").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Forf {
                creature_to_learn_about: Agent::Null.into()
            }
        );
    }
}

#[test]
fn test_command_hair() {
    for p in CaosParser::parse(Rule::command, "HAIR 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Hair { stage: 0.into() }
        );
    }
}

#[test]
fn test_command_injr() {
    for p in CaosParser::parse(Rule::command, "INJR 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Injr {
                organ: 0.into(),
                amount: 1.into()
            }
        );
    }
}

#[test]
fn test_command_like() {
    for p in CaosParser::parse(Rule::command, "LIKE NULL").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Like {
                creature_state_opinion_about: Agent::Null.into()
            }
        );
    }
}

#[test]
fn test_command_loci() {
    for p in CaosParser::parse(Rule::command, "LOCI 1 2 3 4 5").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Loci {
                r#type: 1.into(),
                organ: 2.into(),
                tissue: 3.into(),
                id: 4.into(),
                new_value: 5.into()
            }
        );
    }
}

#[test]
fn test_command_ltcy() {
    for p in CaosParser::parse(Rule::command, "LTCY 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Ltcy {
                action: 1.into(),
                min: 2.into(),
                max: 3.into()
            }
        );
    }
}

#[test]
fn test_command_mate() {
    for p in CaosParser::parse(Rule::command, "MATE").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Mate);
    }
}

#[test]
fn test_command_mvft() {
    for p in CaosParser::parse(Rule::command, "MVFT 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mvft {
                x: 1.into(),
                y: 2.into()
            }
        );
    }
}

#[test]
fn test_command_new_crea() {
    for p in CaosParser::parse(Rule::command, "NEW: CREA 1 NULL 2 3 4").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::NewCrea {
                family: 1.into(),
                gene_agent: Agent::Null.into(),
                gene_slot: 2.into(),
                sex: 3.into(),
                variant: 4.into()
            }
        );
    }
}

#[test]
fn test_command_newc() {
    for p in CaosParser::parse(Rule::command, "NEWC 1 NULL 2 3 4").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Newc {
                family: 1.into(),
                gene_agent: Agent::Null.into(),
                gene_slot: 2.into(),
                sex: 3.into(),
                variant: 4.into()
            }
        );
    }
}

#[test]
fn test_command_norn() {
    for p in CaosParser::parse(Rule::command, "NORN NULL").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Norn {
                creature: Agent::Null.into()
            }
        );
    }
}

#[test]
fn test_command_nude() {
    for p in CaosParser::parse(Rule::command, "NUDE").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Nude);
    }
}

#[test]
fn test_command_ordr_shou() {
    for p in CaosParser::parse(Rule::command, "ORDR SHOU HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::OrdrShou {
                speech: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_ordr_sign() {
    for p in CaosParser::parse(Rule::command, "ORDR SIGN HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::OrdrSign {
                speech: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_ordr_writ() {
    for p in CaosParser::parse(Rule::command, "ORDR WRIT NULL HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::OrdrWrit {
                creature: Agent::Null.into(),
                speech: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_sayn() {
    for p in CaosParser::parse(Rule::command, "SAYN").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Sayn);
    }
}

#[test]
fn test_command_spnl() {
    for p in CaosParser::parse(Rule::command, "SPNL HAND 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Spnl {
                lobe_monkier: SString::Hand.into(),
                neuron_id: 0.into(),
                value: 1.into()
            }
        );
    }
}

#[test]
fn test_command_stim_shou() {
    for p in CaosParser::parse(Rule::command, "STIM SHOU 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::StimShou {
                stimulus: 0.into(),
                strength: 1.into()
            }
        );
    }
}

#[test]
fn test_command_stim_sign() {
    for p in CaosParser::parse(Rule::command, "STIM SIGN 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::StimSign {
                stimulus: 0.into(),
                strength: 1.into()
            }
        );
    }
}

#[test]
fn test_command_stim_tact() {
    for p in CaosParser::parse(Rule::command, "STIM TACT 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::StimTact {
                stimulus: 0.into(),
                strength: 1.into()
            }
        );
    }
}

#[test]
fn test_command_stim_writ() {
    for p in CaosParser::parse(Rule::command, "STIM WRIT NULL 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::StimWrit {
                creature: Agent::Null.into(),
                stimulus: 0.into(),
                strength: 1.into()
            }
        );
    }
}

#[test]
fn test_command_sway_shou() {
    for p in CaosParser::parse(Rule::command, "SWAY SHOU 0 1 2 3 4 5 6 7").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::SwayShou {
                drive1: 0.into(),
                adjust1: 1.into(),
                drive2: 2.into(),
                adjust2: 3.into(),
                drive3: 4.into(),
                adjust3: 5.into(),
                drive4: 6.into(),
                adjust4: 7.into()
            }
        );
    }
}

#[test]
fn test_command_sway_sign() {
    for p in CaosParser::parse(Rule::command, "SWAY SIGN 0 1 2 3 4 5 6 7").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::SwaySign {
                drive1: 0.into(),
                adjust1: 1.into(),
                drive2: 2.into(),
                adjust2: 3.into(),
                drive3: 4.into(),
                adjust3: 5.into(),
                drive4: 6.into(),
                adjust4: 7.into()
            }
        );
    }
}

#[test]
fn test_command_sway_tact() {
    for p in CaosParser::parse(Rule::command, "SWAY TACT 0 1 2 3 4 5 6 7").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::SwayTact {
                drive1: 0.into(),
                adjust1: 1.into(),
                drive2: 2.into(),
                adjust2: 3.into(),
                drive3: 4.into(),
                adjust3: 5.into(),
                drive4: 6.into(),
                adjust4: 7.into()
            }
        );
    }
}

#[test]
fn test_command_sway_writ() {
    for p in CaosParser::parse(Rule::command, "SWAY WRIT NULL 0 1 2 3 4 5 6 7").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::SwayWrit {
                creature: Agent::Null.into(),
                drive1: 0.into(),
                adjust1: 1.into(),
                drive2: 2.into(),
                adjust2: 3.into(),
                drive3: 4.into(),
                adjust3: 5.into(),
                drive4: 6.into(),
                adjust4: 7.into()
            }
        );
    }
}

#[test]
fn test_command_touc() {
    for p in CaosParser::parse(Rule::command, "TOUC").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Touc);
    }
}

#[test]
fn test_command_uncs() {
    for p in CaosParser::parse(Rule::command, "UNCS 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Uncs {
                unconscious: 0.into()
            }
        );
    }
}

#[test]
fn test_command_urge_shou() {
    for p in CaosParser::parse(Rule::command, "URGE SHOU 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::UrgeShou {
                noun_stim: 0.into(),
                verb_id: 1.into(),
                verb_stim: 2.into()
            }
        );
    }
}

#[test]
fn test_command_urge_sign() {
    for p in CaosParser::parse(Rule::command, "URGE SIGN 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::UrgeSign {
                noun_stim: 0.into(),
                verb_id: 1.into(),
                verb_stim: 2.into()
            }
        );
    }
}

#[test]
fn test_command_urge_tact() {
    for p in CaosParser::parse(Rule::command, "URGE TACT 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::UrgeTact {
                noun_stim: 0.into(),
                verb_id: 1.into(),
                verb_stim: 2.into()
            }
        );
    }
}

#[test]
fn test_command_urge_writ() {
    for p in CaosParser::parse(Rule::command, "URGE WRIT NULL 3 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::UrgeWrit {
                creature: Agent::Null.into(),
                noun_id: 3.into(),
                noun_stim: 0.into(),
                verb_id: 1.into(),
                verb_stim: 2.into()
            }
        );
    }
}

#[test]
fn test_command_vocb() {
    for p in CaosParser::parse(Rule::command, "VOCB").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Vocb);
    }
}

#[test]
fn test_command_walk() {
    for p in CaosParser::parse(Rule::command, "WALK").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Walk);
    }
}

#[test]
fn test_command_wear() {
    for p in CaosParser::parse(Rule::command, "WEAR 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Wear {
                body_id: 0.into(),
                set_number: 1.into(),
                layer: 2.into()
            }
        );
    }
}

#[test]
fn test_command_zomb() {
    for p in CaosParser::parse(Rule::command, "ZOMB 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Zomb { zombie: 0.into() }
        );
    }
}

#[test]
fn test_command_apro() {
    for p in CaosParser::parse(Rule::command, "APRO HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Apro {
                search_text: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_dbg_asrt() {
    for p in CaosParser::parse(Rule::command, "DBG: ASRT 1<>2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::DbgAsrt {
                condition: Condition::Simple {
                    cond_type: crate::ast::ConditionType::Ne,
                    lhs: 1.into(),
                    rhs: 2.into()
                }
            }
        );
    }
}

#[test]
fn test_command_dbg_cpro() {
    for p in CaosParser::parse(Rule::command, "DBG: CPRO").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::DbgCpro);
    }
}

#[test]
fn test_command_dbg_flsh() {
    for p in CaosParser::parse(Rule::command, "DBG: FLSH").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::DbgFlsh);
    }
}

#[test]
fn test_command_dbg_html() {
    for p in CaosParser::parse(Rule::command, "DBG: HTML 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::DbgHtml {
                sort_order: 0.into()
            }
        );
    }
}

#[test]
fn test_command_dbg_outs() {
    for p in CaosParser::parse(Rule::command, "DBG: OUTS HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::DbgOuts {
                value: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_dbg_outv() {
    for p in CaosParser::parse(Rule::command, "DBG: OUTV 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::DbgOutv { value: 0.into() }
        );
    }
}

#[test]
fn test_command_dbg_paws() {
    for p in CaosParser::parse(Rule::command, "DBG: PAWS").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::DbgPaws);
    }
}

#[test]
fn test_command_dbg_play() {
    for p in CaosParser::parse(Rule::command, "DBG: PLAY").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::DbgPlay);
    }
}

#[test]
fn test_command_dbg_poll() {
    for p in CaosParser::parse(Rule::command, "DBG: POLL").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::DbgPoll);
    }
}

#[test]
fn test_command_dbg_prof() {
    for p in CaosParser::parse(Rule::command, "DBG: PROF").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::DbgProf);
    }
}

#[test]
fn test_command_dbg_tack() {
    for p in CaosParser::parse(Rule::command, "DBG: TACK NULL").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::DbgTack {
                follow: Agent::Null.into()
            }
        );
    }
}

#[test]
fn test_command_dbg_tock() {
    for p in CaosParser::parse(Rule::command, "DBG: TOCK").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::DbgTock);
    }
}

#[test]
fn test_command_dbg_wtik() {
    for p in CaosParser::parse(Rule::command, "DBG: WTIK 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::DbgWtik {
                new_world_tick: 0.into()
            }
        );
    }
}

#[test]
fn test_command_help() {
    for p in CaosParser::parse(Rule::command, "HELP").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Help);
    }
}

#[test]
fn test_command_mann() {
    for p in CaosParser::parse(Rule::command, "MANN HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mann {
                command: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_memx() {
    for p in CaosParser::parse(Rule::command, "MEMX").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Memx);
    }
}

#[test]
fn test_command_file_glob() {
    for p in CaosParser::parse(Rule::command, "FILE GLOB 0 HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::FileGlob {
                directory: 0.into(),
                file_spec: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_file_iclo() {
    for p in CaosParser::parse(Rule::command, "FILE ICLO").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::FileIclo);
    }
}

#[test]
fn test_command_file_iope() {
    for p in CaosParser::parse(Rule::command, "FILE IOPE 0 HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::FileIope {
                directory: 0.into(),
                filename: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_file_jdel() {
    for p in CaosParser::parse(Rule::command, "FILE JDEL 0 HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::FileJdel {
                directory: 0.into(),
                filename: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_file_oclo() {
    for p in CaosParser::parse(Rule::command, "FILE OCLO").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::FileOclo);
    }
}

#[test]
fn test_command_file_oflu() {
    for p in CaosParser::parse(Rule::command, "FILE OFLU").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::FileOflu);
    }
}

#[test]
fn test_command_file_oope() {
    for p in CaosParser::parse(Rule::command, "FILE OOPE 0 HAND 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::FileOope {
                directory: 0.into(),
                filename: SString::Hand.into(),
                append: 1.into()
            }
        );
    }
}

#[test]
fn test_command_outs() {
    for p in CaosParser::parse(Rule::command, "OUTS HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Outs {
                text: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_outv() {
    for p in CaosParser::parse(Rule::command, "OUTV 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Outv { value: 0.into() }
        );
    }
}

#[test]
fn test_command_outx() {
    for p in CaosParser::parse(Rule::command, "OUTX HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Outx {
                text: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_goto() {
    for p in CaosParser::parse(Rule::command, "GOTO FOOBAR").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Goto {
                destination: String::from("FOOBAR").into()
            }
        );
    }
}

#[test]
fn test_command_gsub() {
    for p in CaosParser::parse(Rule::command, "GSUB FOOBAR").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Gsub {
                destination: String::from("FOOBAR").into()
            }
        );
    }
}

#[test]
fn test_command_gene_clon() {
    for p in CaosParser::parse(Rule::command, "GENE CLON NULL 0 NORN 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::GeneClon {
                dest_agent: Agent::Null.into(),
                dest_slot: 0.into(),
                source_agent: Agent::Norn.into(),
                source_slot: 1.into()
            }
        );
    }
}

#[test]
fn test_command_gene_cros() {
    for p in
        CaosParser::parse(Rule::command, "GENE CROS NULL 0 NORN 1 FROM 2 3 4 5 6").expect("Parsed")
    {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::GeneCros {
                child_agent: Agent::Null.into(),
                child_slot: 0.into(),
                mum_agent: Agent::Norn.into(),
                mum_slot: 1.into(),
                dad_agent: Agent::From.into(),
                dad_slot: 2.into(),
                mum_chance_of_mutation: 3.into(),
                mum_degree_of_mutation: 4.into(),
                dad_chance_of_mutation: 5.into(),
                dad_degree_of_mutation: 6.into()
            }
        );
    }
}

#[test]
fn test_command_gene_kill() {
    for p in CaosParser::parse(Rule::command, "GENE KILL NULL 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::GeneKill {
                agent: Agent::Null.into(),
                slot: 0.into()
            }
        );
    }
}

#[test]
fn test_command_gene_load() {
    for p in CaosParser::parse(Rule::command, "GENE LOAD NULL 0 HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::GeneLoad {
                agent: Agent::Null.into(),
                slot: 0.into(),
                gene_file: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_gene_move() {
    for p in CaosParser::parse(Rule::command, "GENE MOVE NULL 0 NORN 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::GeneMove {
                dest_agent: Agent::Null.into(),
                dest_slot: 0.into(),
                source_agent: Agent::Norn.into(),
                source_slot: 1.into()
            }
        );
    }
}

#[test]
fn test_command_hist_evnt() {
    for p in CaosParser::parse(Rule::command, "HIST EVNT HAND 0 VOIS EMID").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::HistEvnt {
                moniker: SString::Hand.into(),
                event_type: 0.into(),
                related_moniker_1: SString::Vois.into(),
                related_moniker_2: SString::Emid.into()
            }
        );
    }
}

#[test]
fn test_command_hist_foto() {
    for p in CaosParser::parse(Rule::command, "HIST FOTO HAND 0 VOIS").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::HistFoto {
                moniker: SString::Hand.into(),
                event_no: 0.into(),
                new_value: SString::Vois.into()
            }
        );
    }
}

#[test]
fn test_command_hist_name() {
    for p in CaosParser::parse(Rule::command, "HIST NAME HAND VOIS").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::HistName {
                moniker: SString::Hand.into(),
                new_name: SString::Vois.into()
            }
        );
    }
}

#[test]
fn test_command_hist_utxt() {
    for p in CaosParser::parse(Rule::command, "HIST UTXT HAND 0 VOIS").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::HistUtxt {
                moniker: SString::Hand.into(),
                event_no: 0.into(),
                new_value: SString::Vois.into()
            }
        );
    }
}

#[test]
fn test_command_hist_wipe() {
    for p in CaosParser::parse(Rule::command, "HIST WIPE VOIS").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::HistWipe {
                moniker: SString::Vois.into()
            }
        );
    }
}

#[test]
fn test_command_clac() {
    for p in CaosParser::parse(Rule::command, "CLAC 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Clac { message: 0.into() }
        );
    }
}

#[test]
fn test_command_clik() {
    for p in CaosParser::parse(Rule::command, "CLIK 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Clik {
                message_1: 0.into(),
                message_2: 1.into(),
                message_3: 2.into()
            }
        );
    }
}

#[test]
fn test_command_imsk() {
    for p in CaosParser::parse(Rule::command, "IMSK 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Imsk { mask: 0.into() }
        );
    }
}

#[test]
fn test_command_mous() {
    for p in CaosParser::parse(Rule::command, "MOUS 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mous {
                behaviour: 0.into()
            }
        );
    }
}

#[test]
fn test_command_pure() {
    for p in CaosParser::parse(Rule::command, "PURE 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Pure { value: 0.into() }
        );
    }
}

#[test]
fn test_command_tran() {
    for p in CaosParser::parse(Rule::command, "TRAN 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Tran {
                transparency: 0.into(),
                part_no: 1.into()
            }
        );
    }
}

#[test]
fn test_command_addb() {
    for p in CaosParser::parse(Rule::command, "ADDB 0 HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Addb {
                metaroom_id: 0.into(),
                background_file: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_altr() {
    for p in CaosParser::parse(Rule::command, "ALTR 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Altr {
                room_id: 0.into(),
                ca_index: 1.into(),
                ca_delta: 2.into()
            }
        );
    }
}

#[test]
fn test_command_calc() {
    for p in CaosParser::parse(Rule::command, "CALC 0 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Cacl {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into(),
                ca_index: 3.into()
            }
        );
    }
}

#[test]
fn test_command_delm() {
    for p in CaosParser::parse(Rule::command, "DELM 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Delm {
                metaroom_id: 0.into()
            }
        );
    }
}

#[test]
fn test_command_delr() {
    for p in CaosParser::parse(Rule::command, "DELR 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Delr { room_id: 0.into() }
        );
    }
}

#[test]
fn test_command_dmap() {
    for p in CaosParser::parse(Rule::command, "DMAP 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Dmap {
                debug_map: 0.into()
            }
        );
    }
}

#[test]
fn test_command_doca() {
    for p in CaosParser::parse(Rule::command, "DOCA 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Doca {
                no_of_updates: 0.into()
            }
        );
    }
}

#[test]
fn test_command_door() {
    for p in CaosParser::parse(Rule::command, "DOOR 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Door {
                room_id1: 0.into(),
                room_id2: 1.into(),
                permiability: 2.into()
            }
        );
    }
}

#[test]
fn test_command_emit() {
    for p in CaosParser::parse(Rule::command, "EMIT 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Emit {
                ca_index: 0.into(),
                amount: 1.into()
            }
        );
    }
}

#[test]
fn test_command_link() {
    for p in CaosParser::parse(Rule::command, "LINK 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Link {
                room1: 0.into(),
                room2: 1.into(),
                permiability: 2.into()
            }
        );
    }
}

#[test]
fn test_command_mapd() {
    for p in CaosParser::parse(Rule::command, "MAPD 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mapd {
                width: 0.into(),
                height: 1.into()
            }
        );
    }
}

#[test]
fn test_command_mapk() {
    for p in CaosParser::parse(Rule::command, "MAPK").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Mapk);
    }
}

#[test]
fn test_command_perm() {
    for p in CaosParser::parse(Rule::command, "PERM 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Perm {
                permiability: 0.into()
            }
        );
    }
}

#[test]
fn test_command_prop() {
    for p in CaosParser::parse(Rule::command, "PROP 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Prop {
                room_id: 0.into(),
                ca_index: 1.into(),
                ca_value: 2.into()
            }
        );
    }
}

#[test]
fn test_command_rate() {
    for p in CaosParser::parse(Rule::command, "RATE 0 1 2 3 4").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Rate {
                room_type: 0.into(),
                ca_index: 1.into(),
                gain: 2.into(),
                loss: 3.into(),
                diffusion: 4.into()
            }
        );
    }
}

#[test]
fn test_command_rtyp() {
    for p in CaosParser::parse(Rule::command, "RTYP 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Rtyp {
                room_id: 0.into(),
                room_type: 1.into()
            }
        );
    }
}

#[test]
fn test_command_accg() {
    for p in CaosParser::parse(Rule::command, "ACCG 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Accg {
                acceleration: 0.into()
            }
        );
    }
}

#[test]
fn test_command_aero() {
    for p in CaosParser::parse(Rule::command, "AERO 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Aero {
                aerodynamics: 0.into()
            }
        );
    }
}

#[test]
fn test_command_elas() {
    for p in CaosParser::parse(Rule::command, "ELAS 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Elas {
                elasticity: 0.into()
            }
        );
    }
}

#[test]
fn test_command_flto() {
    for p in CaosParser::parse(Rule::command, "FLTO 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Flto {
                screen_x: 0.into(),
                screen_y: 1.into()
            }
        );
    }
}

#[test]
fn test_command_frel() {
    for p in CaosParser::parse(Rule::command, "FREL NULL").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Frel {
                relative: Agent::Null.into()
            }
        );
    }
}

#[test]
fn test_command_fric() {
    for p in CaosParser::parse(Rule::command, "FRIC 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Fric { friction: 0.into() }
        );
    }
}

#[test]
fn test_command_mvby() {
    for p in CaosParser::parse(Rule::command, "MVBY 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mvby {
                delta_x: 0.into(),
                delta_y: 1.into()
            }
        );
    }
}

#[test]
fn test_command_mvsf() {
    for p in CaosParser::parse(Rule::command, "MVSF 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mvsf {
                x: 0.into(),
                y: 1.into()
            }
        );
    }
}

#[test]
fn test_command_mvto() {
    for p in CaosParser::parse(Rule::command, "MVTO 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mvto {
                x: 0.into(),
                y: 1.into()
            }
        );
    }
}

#[test]
fn test_command_velo() {
    for p in CaosParser::parse(Rule::command, "VELO 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Velo {
                x_velocity: 0.into(),
                y_velocity: 1.into()
            }
        );
    }
}

#[test]
fn test_command_prt_bang() {
    for p in CaosParser::parse(Rule::command, "PRT: BANG 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PrtBang {
                bang_strength: 0.into()
            }
        );
    }
}

#[test]
fn test_command_prt_inew() {
    for p in CaosParser::parse(Rule::command, "PRT: INEW 0 HAND VOIS 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PrtInew {
                id: 0.into(),
                name: SString::Hand.into(),
                description: SString::Vois.into(),
                x: 1.into(),
                y: 2.into(),
                message_num: 3.into()
            }
        );
    }
}

#[test]
fn test_command_prt_izap() {
    for p in CaosParser::parse(Rule::command, "PRT: IZAP 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PrtIzap { id: 0.into() }
        );
    }
}

#[test]
fn test_command_prt_join() {
    for p in CaosParser::parse(Rule::command, "PRT: JOIN NULL 0 NORN 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PrtJoin {
                source_agent: Agent::Null.into(),
                output_id: 0.into(),
                dest_agent: Agent::Norn.into(),
                input_id: 2.into()
            }
        );
    }
}

#[test]
fn test_command_prt_krak() {
    for p in CaosParser::parse(Rule::command, "PRT: KRAK NULL 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PrtKrak {
                agent: Agent::Null.into(),
                in_or_out: 0.into(),
                port_index: 1.into()
            }
        );
    }
}

#[test]
fn test_command_prt_onew() {
    for p in CaosParser::parse(Rule::command, "PRT: ONEW 0 HAND VOIS 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PrtOnew {
                id: 0.into(),
                name: SString::Hand.into(),
                description: SString::Vois.into(),
                x: 1.into(),
                y: 2.into()
            }
        );
    }
}

#[test]
fn test_command_prt_ozap() {
    for p in CaosParser::parse(Rule::command, "PRT: OZAP 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PrtOzap { id: 0.into() }
        );
    }
}

#[test]
fn test_command_prt_send() {
    for p in CaosParser::parse(Rule::command, "PRT: SEND 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PrtSend {
                id: 0.into(),
                data: 1.into()
            }
        );
    }
}

#[test]
fn test_command_pray_garb() {
    for p in CaosParser::parse(Rule::command, "PRAY GARB 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::PrayGarb { force: 0.into() }
        );
    }
}

#[test]
fn test_command_pray_refr() {
    for p in CaosParser::parse(Rule::command, "PRAY REFR").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::PrayRefr);
    }
}

#[test]
fn test_command_gids_fmly() {
    for p in CaosParser::parse(Rule::command, "GIDS FMLY 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::GidsFmly { family: 0.into() }
        );
    }
}

#[test]
fn test_command_gids_gnus() {
    for p in CaosParser::parse(Rule::command, "GIDS GNUS 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::GidsGnus {
                family: 0.into(),
                genus: 1.into()
            }
        );
    }
}

#[test]
fn test_command_gids_root() {
    for p in CaosParser::parse(Rule::command, "GIDS ROOT").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::GidsRoot);
    }
}

#[test]
fn test_command_gids_spec() {
    for p in CaosParser::parse(Rule::command, "GIDS SPEC 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::GidsSpcs {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into()
            }
        );
    }
}

#[test]
fn test_command_inst() {
    for p in CaosParser::parse(Rule::command, "INST").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Inst);
    }
}

#[test]
fn test_command_lock() {
    for p in CaosParser::parse(Rule::command, "LOCK").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Lock);
    }
}

#[test]
fn test_command_scrx() {
    for p in CaosParser::parse(Rule::command, "SCRX 0 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Scrx {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into(),
                event: 3.into()
            }
        );
    }
}

#[test]
fn test_command_slow() {
    for p in CaosParser::parse(Rule::command, "SLOW").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Slow);
    }
}

#[test]
fn test_command_sorc() {
    for p in CaosParser::parse(Rule::command, "SORC 0 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Sorc {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into(),
                event: 3.into()
            }
        );
    }
}

#[test]
fn test_command_stop() {
    for p in CaosParser::parse(Rule::command, "STOP").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Stop);
    }
}

#[test]
fn test_command_stpt() {
    for p in CaosParser::parse(Rule::command, "STPT").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Stpt);
    }
}

#[test]
fn test_command_unlk() {
    for p in CaosParser::parse(Rule::command, "UNLK").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Unlk);
    }
}

#[test]
fn test_command_wait() {
    for p in CaosParser::parse(Rule::command, "WAIT 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Wait { ticks: 0.into() }
        );
    }
}

#[test]
fn test_command_fade() {
    for p in CaosParser::parse(Rule::command, "FADE").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Fade);
    }
}

#[test]
fn test_command_mclr() {
    for p in CaosParser::parse(Rule::command, "MCLR 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mclr {
                x: 0.into(),
                y: 1.into()
            }
        );
    }
}

#[test]
fn test_command_midi() {
    for p in CaosParser::parse(Rule::command, "MIDI HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Midi {
                midi_file: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_mmsc() {
    for p in CaosParser::parse(Rule::command, "MMSC 0 1 HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mmsc {
                x: 0.into(),
                y: 1.into(),
                track_name: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_rclr() {
    for p in CaosParser::parse(Rule::command, "RCLR 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Rclr {
                x: 0.into(),
                y: 1.into()
            }
        );
    }
}

#[test]
fn test_command_rmsc() {
    for p in CaosParser::parse(Rule::command, "RMSC 0 1 HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Rmsc {
                x: 0.into(),
                y: 1.into(),
                track_name: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_sezz() {
    for p in CaosParser::parse(Rule::command, "SEZZ HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Sezz {
                text: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_sndc() {
    for p in CaosParser::parse(Rule::command, "SNDC HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Sndc {
                sound_file: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_snde() {
    for p in CaosParser::parse(Rule::command, "SNDE HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Snde {
                sound_file: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_sndl() {
    for p in CaosParser::parse(Rule::command, "SNDL HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Sndl {
                sound_file: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_sndq() {
    for p in CaosParser::parse(Rule::command, "SNDQ HAND 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Sndq {
                sound_file: SString::Hand.into(),
                delay: 0.into()
            }
        );
    }
}

#[test]
fn test_command_stpc() {
    for p in CaosParser::parse(Rule::command, "STPC").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Stpc);
    }
}

#[test]
fn test_command_strk() {
    for p in CaosParser::parse(Rule::command, "STRK 0 HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Strk {
                latency: 0.into(),
                track: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_voic() {
    for p in CaosParser::parse(Rule::command, "VOIC 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Voic {
                genus: 0.into(),
                gender: 1.into(),
                age: 2.into()
            }
        );
    }
}

#[test]
fn test_command_vois() {
    for p in CaosParser::parse(Rule::command, "VOIS HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Vois {
                voice_name: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_volm() {
    for p in CaosParser::parse(Rule::command, "VOLM 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Volm { volume: 0.into() }
        );
    }
}

#[test]
fn test_command_wpau() {
    for p in CaosParser::parse(Rule::command, "WPAU 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Wpau { paused: 0.into() }
        );
    }
}

#[test]
fn test_command_absv() {
    for p in CaosParser::parse(Rule::command, "ABSV VELX").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Absv {
                var: Variable::Velx
            }
        );
    }
}

#[test]
fn test_command_adds() {
    for p in CaosParser::parse(Rule::command, "ADDS VELX HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Adds {
                var: Variable::Velx,
                append: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_addv() {
    for p in CaosParser::parse(Rule::command, "ADDV VELX 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Addv {
                var: Variable::Velx,
                sum: 0.into()
            }
        );
    }
}

#[test]
fn test_command_andv() {
    for p in CaosParser::parse(Rule::command, "ANDV VELX 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Andv {
                var: Variable::Velx,
                value: 0.into()
            }
        );
    }
}

#[test]
fn test_command_char() {
    for p in CaosParser::parse(Rule::command, "CHAR VELX 0 1").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Char {
                string: Variable::Velx,
                index: 0.into(),
                character: 1.into()
            }
        );
    }
}

#[test]
fn test_command_delg() {
    for p in CaosParser::parse(Rule::command, "DELG HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Delg {
                variable_name: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_divv() {
    for p in CaosParser::parse(Rule::command, "DIVV VELX 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Divv {
                var: Variable::Velx,
                div: 0.into()
            }
        );
    }
}

#[test]
fn test_command_modv() {
    for p in CaosParser::parse(Rule::command, "MODV VELX 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Modv {
                var: Variable::Velx,
                r#mod: 0.into()
            }
        );
    }
}

#[test]
fn test_command_mulv() {
    for p in CaosParser::parse(Rule::command, "MULV VELX 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Mulv {
                var: Variable::Velx,
                mul: 0.into()
            }
        );
    }
}

#[test]
fn test_command_negv() {
    for p in CaosParser::parse(Rule::command, "NEGV VELX").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Negv {
                var: Variable::Velx
            }
        );
    }
}

#[test]
fn test_command_orrv() {
    for p in CaosParser::parse(Rule::command, "ORRV VELX 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Orrv {
                var: Variable::Velx,
                value: 0.into()
            }
        );
    }
}

#[test]
fn test_command_reaf() {
    for p in CaosParser::parse(Rule::command, "REAF").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Reaf);
    }
}

#[test]
fn test_command_seta() {
    for p in CaosParser::parse(Rule::command, "SETA VELX NULL").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Seta {
                var: Variable::Velx,
                value: Agent::Null.into()
            }
        );
    }
}

#[test]
fn test_command_sets() {
    for p in CaosParser::parse(Rule::command, "SETS VELX HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Sets {
                var: Variable::Velx,
                value: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_setv() {
    for p in CaosParser::parse(Rule::command, "SETV VELX 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Setv {
                var: Variable::Velx,
                value: 0.into()
            }
        );
    }
}

#[test]
fn test_command_subv() {
    for p in CaosParser::parse(Rule::command, "SUBV VELX 0)").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Subv {
                var: Variable::Velx,
                sub: 0.into()
            }
        );
    }
}

#[test]
fn test_command_targ() {
    for p in CaosParser::parse(Rule::command, "TARG NULL").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Targ {
                agent: Agent::Null.into()
            }
        );
    }
}

#[test]
fn test_command_cabn() {
    for p in CaosParser::parse(Rule::command, "CABN 0 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Cabn {
                left: 0.into(),
                top: 1.into(),
                right: 2.into(),
                bottom: 3.into()
            }
        );
    }
}

#[test]
fn test_command_cabp() {
    for p in CaosParser::parse(Rule::command, "CABP 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Cabp { plane: 0.into() }
        );
    }
}

#[test]
fn test_command_cabv() {
    for p in CaosParser::parse(Rule::command, "CABV 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Cabv {
                cabin_room_id: 0.into()
            }
        );
    }
}

#[test]
fn test_command_cabw() {
    for p in CaosParser::parse(Rule::command, "CABW 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Cabw {
                cabin_capacity: 0.into()
            }
        );
    }
}

#[test]
fn test_command_dpas() {
    for p in CaosParser::parse(Rule::command, "DPAS 0 1 2").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Dpas {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into()
            }
        );
    }
}

#[test]
fn test_command_gpas() {
    for p in CaosParser::parse(Rule::command, "GPAS 0 1 2 3").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Gpas {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into(),
                rect_to_use: 3.into()
            }
        );
    }
}

#[test]
fn test_command_new_vhcl() {
    for p in CaosParser::parse(Rule::command, "NEW: VHCL 0 1 2 HAND 3 4 5").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::NewVhcl {
                family: 0.into(),
                genus: 1.into(),
                species: 2.into(),
                sprite_file: SString::Hand.into(),
                image_count: 3.into(),
                first_image: 4.into(),
                plane: 5.into()
            }
        );
    }
}

#[test]
fn test_command_rpas() {
    for p in CaosParser::parse(Rule::command, "RPAS NULL NORN").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Rpas {
                vehicle: Agent::Null.into(),
                passenger: Agent::Norn.into()
            }
        );
    }
}

#[test]
fn test_command_spas() {
    for p in CaosParser::parse(Rule::command, "SPAS NULL NORN").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Spas {
                vehicle: Agent::Null.into(),
                new_passenger: Agent::Norn.into()
            }
        );
    }
}

#[test]
fn test_command_delw() {
    for p in CaosParser::parse(Rule::command, "DELW HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Delw {
                world_name: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_load() {
    for p in CaosParser::parse(Rule::command, "LOAD HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Load {
                world_name: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_pswd() {
    for p in CaosParser::parse(Rule::command, "PSWD HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Pswd {
                world_name: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_quit() {
    for p in CaosParser::parse(Rule::command, "QUIT").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Quit);
    }
}

#[test]
fn test_command_rgam() {
    for p in CaosParser::parse(Rule::command, "RGAM").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Rgam);
    }
}

#[test]
fn test_command_save() {
    for p in CaosParser::parse(Rule::command, "SAVE").expect("Parsed") {
        assert_eq!(parse_command(p).expect("Parsed command"), Command::Save);
    }
}

#[test]
fn test_command_tntw() {
    for p in CaosParser::parse(Rule::command, "TNTW 0").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Tntw { index: 0.into() }
        );
    }
}

#[test]
fn test_command_wrld() {
    for p in CaosParser::parse(Rule::command, "WRLD HAND").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Wrld {
                world_name: SString::Hand.into()
            }
        );
    }
}

#[test]
fn test_command_wtnt() {
    for p in CaosParser::parse(Rule::command, "WTNT 1 2 3 4 5 6").expect("Parsed") {
        assert_eq!(
            parse_command(p).expect("Parsed command"),
            Command::Wtnt {
                index: 1.into(),
                red_tint: 2.into(),
                green_tint: 3.into(),
                blue_tint: 4.into(),
                rotation: 5.into(),
                swap: 6.into()
            }
        );
    }
}
