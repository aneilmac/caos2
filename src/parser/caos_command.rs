mod doif;
mod r#enum;
mod r#loop;
mod subr;

use super::{
    condition::parse_anything, parse_agent_arg, parse_bytestring_literal, parse_float_arg,
    parse_int_arg, parse_string_arg,
};
use crate::{ast::Command, CaosError, ErrorType, Rule};
use doif::*;
use pest::iterators::Pair;
use r#enum::*;
use r#loop::*;
use subr::*;

pub fn parse_command(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::command_doif => parse_command_doif(pair),
        Rule::command_subr => parse_command_subr(pair),
        Rule::command_reps => parse_command_reps(pair),
        Rule::command_econ => parse_command_econ(pair),
        Rule::command_enum => parse_command_enum(pair),
        Rule::command_etch => parse_command_etch(pair),
        Rule::command_esee => parse_command_esee(pair),
        Rule::command_epass => parse_command_epass(pair),
        Rule::command_loop_untl => parse_command_loop_untl(pair),
        Rule::command_loop_ever => parse_command_loop_ever(pair),
        Rule::command_anim => {
            let mut it = pair.clone().into_inner();
            let pose_list = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_bytestring_literal)?;
            Ok(Command::Anim { pose_list })
        }
        Rule::command_anms => {
            let mut it = pair.clone().into_inner();
            let anim_string = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Anms { anim_string })
        }
        Rule::command_attr => {
            let mut it = pair.clone().into_inner();
            let attributes = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Attr { attributes })
        }
        Rule::command_base => {
            let mut it = pair.clone().into_inner();
            let index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Base { index })
        }
        Rule::command_bhvr => {
            let mut it = pair.clone().into_inner();
            let permissions = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Bhvr { permissions })
        }
        Rule::command_frat => {
            let mut it = pair.clone().into_inner();
            let framerate = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Frat { framerate })
        }
        Rule::command_gait => {
            let mut it = pair.clone().into_inner();
            let gait_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Gait { gait_number })
        }
        Rule::command_gall => {
            let mut it = pair.clone().into_inner();
            let sprite_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let first_image = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Gall {
                sprite_file,
                first_image,
            })
        }
        Rule::command_hand => {
            let mut it = pair.clone().into_inner();
            let name_for_the_hand = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Hand { name_for_the_hand })
        }
        Rule::command_kill => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::Kill { agent })
        }
        Rule::command_mesg_writ => {
            let mut it = pair.clone().into_inner();
            let command = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let message_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::MesgWrit {
                command,
                message_id,
            })
        }
        Rule::command_mesg_wrt => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let message_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let param_1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_anything)?;
            let param_2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_anything)?;
            let delay = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::MesgWritPlus {
                agent,
                message_id,
                param_1,
                param_2,
                delay,
            })
        }
        Rule::int_mira => {
            let mut it = pair.clone().into_inner();
            let on_off = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Mira { on_off })
        }
        Rule::command_new_simp => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let species = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let sprite_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let image_count = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let first_image = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let plane = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::NewSimp {
                family,
                genus,
                species,
                sprite_file,
                image_count,
                first_image,
                plane,
            })
        }
        Rule::command_nohh => Ok(Command::Nohh),
        Rule::command_over => Ok(Command::Over),
        Rule::command_paus => {
            let mut it = pair.clone().into_inner();
            let paused = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Paus { paused })
        }
        Rule::command_plne => {
            let mut it = pair.clone().into_inner();
            let plane = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Plne { plane })
        }
        Rule::command_pose => {
            let mut it = pair.clone().into_inner();
            let pose = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Pose { pose })
        }
        Rule::command_puhl => {
            let mut it = pair.clone().into_inner();
            let pose = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Puhl { pose, x, y })
        }
        Rule::command_pupt => {
            let mut it = pair.clone().into_inner();
            let pose = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Pupt { pose, x, y })
        }
        Rule::command_rnge => {
            let mut it = pair.clone().into_inner();
            let distance = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Rnge { distance })
        }
        Rule::command_rtar => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let species = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Rtar {
                family,
                genus,
                species,
            })
        }
        Rule::command_show => {
            let mut it = pair.clone().into_inner();
            let visibility = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Show { visibility })
        }
        Rule::command_star => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let species = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Star {
                family,
                genus,
                species,
            })
        }
        Rule::command_tick => {
            let mut it = pair.clone().into_inner();
            let red_tint = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let green_tint = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let blue_tint = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let rotation = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let swap = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Tint {
                red_tint,
                green_tint,
                blue_tint,
                rotation,
                swap,
            })
        }
        Rule::command_ttar => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let species = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Ttar {
                family,
                genus,
                species,
            })
        }
        Rule::command_brn_dmpb => Ok(Command::BrnDmpb),
        Rule::command_brn_dmpd => {
            let mut it = pair.clone().into_inner();
            let tract_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let dendrite_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::BrnDmpd {
                tract_number,
                dendrite_number,
            })
        }
        Rule::command_brn_dmpl => {
            let mut it = pair.clone().into_inner();
            let lobe_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::BrnDmpl { lobe_number })
        }
        Rule::command_brn_dmpn => {
            let mut it = pair.clone().into_inner();
            let lobe_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let neuron_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::BrnDmpn {
                lobe_number,
                neuron_number,
            })
        }
        Rule::command_brn_dmpt => {
            let mut it = pair.clone().into_inner();
            let tract_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::BrnDmpt { tract_number })
        }
        Rule::command_brn_setd => {
            let mut it = pair.clone().into_inner();
            let tract_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let dendrite_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let weight_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let new_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::BrnSetd {
                tract_number,
                dendrite_number,
                weight_number,
                new_value,
            })
        }
        Rule::command_brn_setl => {
            let mut it = pair.clone().into_inner();
            let lobe_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let line_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let new_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::BrntSetl {
                lobe_number,
                line_number,
                new_value,
            })
        }
        Rule::command_brn_setn => {
            let mut it = pair.clone().into_inner();
            let lobe_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let neuron_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let state_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let new_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::BrnSetn {
                lobe_number,
                neuron_number,
                state_number,
                new_value,
            })
        }
        Rule::command_brn_sett => {
            let mut it = pair.clone().into_inner();
            let tract_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let line_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let new_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::BrnSett {
                tract_number,
                line_number,
                new_value,
            })
        }
        Rule::command_bkgd => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let background = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let transition = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Bkgd {
                metaroom_id,
                background,
                transition,
            })
        }
        Rule::command_brmi => {
            let mut it = pair.clone().into_inner();
            let mearoom_base = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let room_base = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Brmi {
                mearoom_base,
                room_base,
            })
        }
        Rule::command_cmra => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let pan = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Cmra { x, y, pan })
        }
        Rule::command_cmrp => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let pan = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Cmrp { x, y, pan })
        }
        Rule::command_cmrt => {
            let mut it = pair.clone().into_inner();
            let pan = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Cmrt { pan })
        }
        Rule::command_frsh => Ok(Command::Frsh),
        Rule::command_line => {
            let mut it = pair.clone().into_inner();
            let x1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let x2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let r = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let g = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let b = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let stipple_on = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let stipple_off = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Line {
                x1,
                y1,
                x2,
                y2,
                r,
                g,
                b,
                stipple_on,
                stipple_off,
            })
        }
        Rule::command_meta => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let camera_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let camera_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let transition = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Meta {
                metaroom_id,
                camera_x,
                camera_y,
                transition,
            })
        }
        Rule::command_scam => {
            let mut it = pair.clone().into_inner();
            let compound_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let part_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Scam {
                compound_agent,
                part_number,
            })
        }
        _ => todo!(),
    }
}
