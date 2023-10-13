mod doif;
mod r#enum;
mod r#loop;
mod subr;
#[cfg(test)]
mod tests;

use super::{
    parse_agent_arg, parse_anything, parse_bytestring_literal, parse_condition, parse_decimal_arg,
    parse_float_arg, parse_int_arg, parse_label, parse_string_arg, parse_variable,
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
        Rule::command_epas => parse_command_epas(pair),
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
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let message_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::MesgWrit { agent, message_id })
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
        Rule::command_mira => {
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
        Rule::command_tick => {
            let mut it = pair.clone().into_inner();
            let tick_rate = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Tick { tick_rate })
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
        Rule::command_tint => {
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
            Ok(Command::BrnSetl {
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
        Rule::command_snap => {
            let mut it = pair.clone().into_inner();
            let filename = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let x_centre = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y_centre = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let width = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let height = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let zoom_factor = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Snap {
                filename,
                x_centre,
                y_centre,
                width,
                height,
                zoom_factor,
            })
        }
        Rule::command_trck => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let x_percent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y_percent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let style = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let transition = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Trck {
                agent,
                x_percent,
                y_percent,
                style,
                transition,
            })
        }
        Rule::command_wdow => Ok(Command::Wdow),
        Rule::command_zoom => {
            let mut it = pair.clone().into_inner();
            let pixels = it
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
            Ok(Command::Zoom { pixels, x, y })
        }
        Rule::command_fcus => Ok(Command::Fcus),
        Rule::command_frmt => {
            let mut it = pair.clone().into_inner();
            let left_margin = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let top_margin = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let right_margin = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let bottom_margin = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let line_spacing = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let character_spacing = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let justification = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Frmt {
                left_margin,
                top_margin,
                right_margin,
                bottom_margin,
                line_spacing,
                character_spacing,
                justification,
            })
        }
        Rule::command_grpl => {
            let mut it = pair.clone().into_inner();
            let red = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let green = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let blue = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let min_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let max_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Grpl {
                red,
                green,
                blue,
                min_y,
                max_y,
            })
        }
        Rule::command_grpv => {
            let mut it = pair.clone().into_inner();
            let line_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Grpv { line_index, value })
        }
        Rule::command_new_comp => {
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
            Ok(Command::NewComp {
                family,
                genus,
                species,
                sprite_file,
                image_count,
                first_image,
                plane,
            })
        }
        Rule::command_page => {
            let mut it = pair.clone().into_inner();
            let page = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Page { page })
        }
        Rule::command_part => {
            let mut it = pair.clone().into_inner();
            let part_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Part { part_id })
        }
        Rule::command_pat_butt => {
            let mut it = pair.clone().into_inner();
            let part_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let sprite_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let first_image = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let image_count = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let rel_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_plane = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let anim_hover = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_bytestring_literal)?;
            let message_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let option = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PatButt {
                part_id,
                sprite_file,
                first_image,
                image_count,
                rel_x,
                rel_y,
                rel_plane,
                anim_hover,
                message_id,
                option,
            })
        }
        Rule::command_pat_cmra => {
            let mut it = pair.clone().into_inner();
            let part_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let overlay_sprite = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let base_image = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let rel_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_plane = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let view_width = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let view_height = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let camera_width = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let camera_height = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PatCmra {
                part_id,
                overlay_sprite,
                base_image,
                rel_x,
                rel_y,
                rel_plane,
                view_width,
                view_height,
                camera_width,
                camera_height,
            })
        }
        Rule::command_pat_dull => {
            let mut it = pair.clone().into_inner();
            let part_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let sprite_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let first_image = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let rel_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_plane = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PatDull {
                part_id,
                sprite_file,
                first_image,
                rel_x,
                rel_y,
                rel_plane,
            })
        }
        Rule::command_pat_fixd => {
            let mut it = pair.clone().into_inner();
            let part_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let sprite_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let first_image = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let rel_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_plane = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let font_sprite = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::PatFixd {
                part_id,
                sprite_file,
                first_image,
                rel_x,
                rel_y,
                rel_plane,
                font_sprite,
            })
        }
        Rule::command_pat_grph => {
            let mut it = pair.clone().into_inner();
            let part_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let overlay_sprite = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let base_image = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let rel_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_plane = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let num_values = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PatGrph {
                part_id,
                overlay_sprite,
                base_image,
                rel_x,
                rel_y,
                rel_plane,
                num_values,
            })
        }
        Rule::command_pat_kill => {
            let mut it = pair.clone().into_inner();
            let part_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PatKill { part_id })
        }
        Rule::command_pat_text => {
            let mut it = pair.clone().into_inner();
            let part_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let sprite_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let first_image = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let rel_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            let rel_plane = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let message_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let font_sprite = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::PatText {
                part_id,
                sprite_file,
                first_image,
                rel_x,
                rel_y,
                rel_plane,
                message_id,
                font_sprite,
            })
        }
        Rule::command_ptxt => {
            let mut it = pair.clone().into_inner();
            let text = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Ptxt { text })
        }
        Rule::command_ages => {
            let mut it = pair.clone().into_inner();
            let times = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Ages { times })
        }
        Rule::command_appr => Ok(Command::Appr),
        Rule::command_aslp => {
            let mut it = pair.clone().into_inner();
            let asleep = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Aslp { asleep })
        }
        Rule::command_body => {
            let mut it = pair.clone().into_inner();
            let set_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let layer = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Body { set_number, layer })
        }
        Rule::command_born => Ok(Command::Born),
        Rule::command_chem => {
            let mut it = pair.clone().into_inner();
            let chemical = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjustment = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Chem {
                chemical,
                adjustment,
            })
        }
        Rule::command_dead => Ok(Command::Dead),
        Rule::command_dirn => {
            let mut it = pair.clone().into_inner();
            let direction = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Dirn { direction })
        }
        Rule::command_done => Ok(Command::Done),
        Rule::command_drea => {
            let mut it = pair.clone().into_inner();
            let dream = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Drea { dream })
        }
        Rule::command_driv => {
            let mut it = pair.clone().into_inner();
            let drive = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjustment = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Driv { drive, adjustment })
        }
        Rule::command_face => {
            let mut it = pair.clone().into_inner();
            let set_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Face { set_number })
        }
        Rule::command_forf => {
            let mut it = pair.clone().into_inner();
            let creature_to_learn_about = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::Forf {
                creature_to_learn_about,
            })
        }
        Rule::command_hair => {
            let mut it = pair.clone().into_inner();
            let stage = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Hair { stage })
        }
        Rule::command_injr => {
            let mut it = pair.clone().into_inner();
            let organ = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let amount = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Injr { organ, amount })
        }
        Rule::command_like => {
            let mut it = pair.clone().into_inner();
            let creature_state_opinion_about = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::Like {
                creature_state_opinion_about,
            })
        }
        Rule::command_loci => {
            let mut it = pair.clone().into_inner();
            let r#type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let organ = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let tissue = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let new_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Loci {
                r#type,
                organ,
                tissue,
                id,
                new_value,
            })
        }
        Rule::command_ltcy => {
            let mut it = pair.clone().into_inner();
            let action = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let min = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let max = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Ltcy { action, min, max })
        }
        Rule::command_mate => Ok(Command::Mate),
        Rule::command_mvft => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Mvft { x, y })
        }
        Rule::command_new_crea => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let gene_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let gene_slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let sex = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let variant = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::NewCrea {
                family,
                gene_agent,
                gene_slot,
                sex,
                variant,
            })
        }
        Rule::command_newc => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let gene_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let gene_slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let sex = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let variant = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Newc {
                family,
                gene_agent,
                gene_slot,
                sex,
                variant,
            })
        }
        Rule::command_norn => {
            let mut it = pair.clone().into_inner();
            let creature = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::Norn { creature })
        }
        Rule::command_nude => Ok(Command::Nude),
        Rule::command_ordr_shou => {
            let mut it = pair.clone().into_inner();
            let speech = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::OrdrShou { speech })
        }
        Rule::command_ordr_sign => {
            let mut it = pair.clone().into_inner();
            let speech = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::OrdrSign { speech })
        }
        Rule::command_ordr_writ => {
            let mut it = pair.clone().into_inner();
            let creature = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let speech = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::OrdrWrit { creature, speech })
        }
        Rule::command_sayn => Ok(Command::Sayn),
        Rule::command_spnl => {
            let mut it = pair.clone().into_inner();
            let lobe_monkier = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let neuron_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Spnl {
                lobe_monkier,
                neuron_id,
                value,
            })
        }
        Rule::command_stim_shou => {
            let mut it = pair.clone().into_inner();
            let stimulus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let strength = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::StimShou { stimulus, strength })
        }
        Rule::command_stim_sign => {
            let mut it = pair.clone().into_inner();
            let stimulus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let strength = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::StimSign { stimulus, strength })
        }
        Rule::command_stim_tact => {
            let mut it = pair.clone().into_inner();
            let stimulus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let strength = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::StimTact { stimulus, strength })
        }
        Rule::command_stim_writ => {
            let mut it = pair.clone().into_inner();
            let creature = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let stimulus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let strength = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::StimWrit {
                creature,
                stimulus,
                strength,
            })
        }
        Rule::command_sway_shou => {
            let mut it = pair.clone().into_inner();
            let drive1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive3 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust3 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive4 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust4 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::SwayShou {
                drive1,
                adjust1,
                drive2,
                adjust2,
                drive3,
                adjust3,
                drive4,
                adjust4,
            })
        }
        Rule::command_sway_sign => {
            let mut it = pair.clone().into_inner();
            let drive1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive3 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust3 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive4 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust4 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::SwaySign {
                drive1,
                adjust1,
                drive2,
                adjust2,
                drive3,
                adjust3,
                drive4,
                adjust4,
            })
        }
        Rule::command_sway_tact => {
            let mut it = pair.clone().into_inner();
            let drive1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive3 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust3 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive4 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust4 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::SwayTact {
                drive1,
                adjust1,
                drive2,
                adjust2,
                drive3,
                adjust3,
                drive4,
                adjust4,
            })
        }
        Rule::command_sway_writ => {
            let mut it = pair.clone().into_inner();
            let creature = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let drive1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive3 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust3 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let drive4 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let adjust4 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::SwayWrit {
                creature,
                drive1,
                adjust1,
                drive2,
                adjust2,
                drive3,
                adjust3,
                drive4,
                adjust4,
            })
        }
        Rule::command_touc => Ok(Command::Touc),
        Rule::command_uncs => {
            let mut it = pair.clone().into_inner();
            let unconscious = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Uncs { unconscious })
        }
        Rule::command_urge_shou => {
            let mut it = pair.clone().into_inner();
            let noun_stim = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let verb_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let verb_stim = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::UrgeShou {
                noun_stim,
                verb_id,
                verb_stim,
            })
        }
        Rule::command_urge_sign => {
            let mut it = pair.clone().into_inner();
            let noun_stim = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let verb_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let verb_stim = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::UrgeSign {
                noun_stim,
                verb_id,
                verb_stim,
            })
        }
        Rule::command_urge_tact => {
            let mut it = pair.clone().into_inner();
            let noun_stim = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let verb_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let verb_stim = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::UrgeTact {
                noun_stim,
                verb_id,
                verb_stim,
            })
        }
        Rule::command_urge_writ => {
            let mut it = pair.clone().into_inner();
            let creature = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let noun_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let noun_stim = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let verb_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let verb_stim = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::UrgeWrit {
                creature,
                noun_id,
                noun_stim,
                verb_id,
                verb_stim,
            })
        }
        Rule::command_vocb => Ok(Command::Vocb),
        Rule::command_walk => Ok(Command::Walk),
        Rule::command_wear => {
            let mut it = pair.clone().into_inner();
            let body_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let set_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let layer = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Wear {
                body_id,
                set_number,
                layer,
            })
        }
        Rule::command_zomb => {
            let mut it = pair.clone().into_inner();
            let zombie = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Zomb { zombie })
        }
        Rule::command_apro => {
            let mut it = pair.clone().into_inner();
            let search_text = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Apro { search_text })
        }
        Rule::command_dbg_asrt => {
            let mut it = pair.clone().into_inner();
            let condition = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_condition)?;
            Ok(Command::DbgAsrt { condition })
        }
        Rule::command_dbg_cpro => Ok(Command::DbgCpro),
        Rule::command_dbg_flsh => Ok(Command::DbgFlsh),
        Rule::command_dbg_html => {
            let mut it = pair.clone().into_inner();
            let sort_order = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::DbgHtml { sort_order })
        }
        Rule::command_dbg_outs => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::DbgOuts { value })
        }
        Rule::command_dbg_outv => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            Ok(Command::DbgOutv { value })
        }
        Rule::command_dbg_paws => Ok(Command::DbgPaws),
        Rule::command_dbg_play => Ok(Command::DbgPlay),
        Rule::command_dbg_poll => Ok(Command::DbgPoll),
        Rule::command_dbg_prof => Ok(Command::DbgProf),
        Rule::command_dbg_tack => {
            let mut it = pair.clone().into_inner();
            let follow = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::DbgTack { follow })
        }
        Rule::command_dbg_tock => Ok(Command::DbgTock),
        Rule::command_dbg_wtik => {
            let mut it = pair.clone().into_inner();
            let new_world_tick = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::DbgWtik { new_world_tick })
        }
        Rule::command_help => Ok(Command::Help),
        Rule::command_mann => {
            let mut it = pair.clone().into_inner();
            let command = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Mann { command })
        }
        Rule::command_memx => Ok(Command::Memx),
        Rule::command_file_glob => {
            let mut it = pair.clone().into_inner();
            let directory = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let file_spec = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::FileGlob {
                directory,
                file_spec,
            })
        }
        Rule::command_file_iclo => Ok(Command::FileIclo),
        Rule::command_file_iope => {
            let mut it = pair.clone().into_inner();
            let directory = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let filename = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::FileIope {
                directory,
                filename,
            })
        }
        Rule::command_file_jdel => {
            let mut it = pair.clone().into_inner();
            let directory = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let filename = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::FileJdel {
                directory,
                filename,
            })
        }
        Rule::command_file_oclo => Ok(Command::FileOclo),
        Rule::command_file_oflu => Ok(Command::FileOflu),
        Rule::command_file_oope => {
            let mut it = pair.clone().into_inner();
            let directory = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let filename = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let append = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::FileOope {
                directory,
                filename,
                append,
            })
        }
        Rule::command_outs => {
            let mut it = pair.clone().into_inner();
            let text = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Outs { text })
        }
        Rule::command_outv => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            Ok(Command::Outv { value })
        }
        Rule::command_outx => {
            let mut it = pair.clone().into_inner();
            let text = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Outx { text })
        }
        Rule::command_goto => {
            let mut it = pair.clone().into_inner();
            let destination = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_label)?;
            Ok(Command::Goto { destination })
        }
        Rule::command_gsub => {
            let mut it = pair.clone().into_inner();
            let destination = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_label)?;
            Ok(Command::Gsub { destination })
        }
        Rule::command_gene_clon => {
            let mut it = pair.clone().into_inner();
            let dest_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let dest_slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let source_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let source_slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::GeneClon {
                dest_agent,
                dest_slot,
                source_agent,
                source_slot,
            })
        }
        Rule::command_gene_cros => {
            let mut it = pair.clone().into_inner();
            let child_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let child_slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let mum_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let mum_slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let dad_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let dad_slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let mum_chance_of_mutation = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let mum_degree_of_mutation = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let dad_chance_of_mutation = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let dad_degree_of_mutation = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::GeneCros {
                child_agent,
                child_slot,
                mum_agent,
                mum_slot,
                dad_agent,
                dad_slot,
                mum_chance_of_mutation,
                mum_degree_of_mutation,
                dad_chance_of_mutation,
                dad_degree_of_mutation,
            })
        }
        Rule::command_gene_kill => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::GeneKill { agent, slot })
        }
        Rule::command_gene_load => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let gene_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::GeneLoad {
                agent,
                slot,
                gene_file,
            })
        }
        Rule::command_gene_move => {
            let mut it = pair.clone().into_inner();
            let dest_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let dest_slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let source_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let source_slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::GeneMove {
                dest_agent,
                dest_slot,
                source_agent,
                source_slot,
            })
        }
        Rule::command_hist_evnt => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let event_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let related_moniker_1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let related_moniker_2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::HistEvnt {
                moniker,
                event_type,
                related_moniker_1,
                related_moniker_2,
            })
        }
        Rule::command_hist_foto => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let new_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::HistFoto {
                moniker,
                event_no,
                new_value,
            })
        }
        Rule::command_hist_name => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let new_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::HistName { moniker, new_name })
        }
        Rule::command_hist_utxt => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let new_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::HistUtxt {
                moniker,
                event_no,
                new_value,
            })
        }
        Rule::command_hist_wipe => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::HistWipe { moniker })
        }
        Rule::command_clac => {
            let mut it = pair.clone().into_inner();
            let message = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Clac { message })
        }
        Rule::command_clik => {
            let mut it = pair.clone().into_inner();
            let message_1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let message_2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let message_3 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Clik {
                message_1,
                message_2,
                message_3,
            })
        }
        Rule::command_imsk => {
            let mut it = pair.clone().into_inner();
            let mask = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Imsk { mask })
        }
        Rule::command_mous => {
            let mut it = pair.clone().into_inner();
            let behaviour = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Mous { behaviour })
        }
        Rule::command_pure => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Pure { value })
        }
        Rule::command_tran => {
            let mut it = pair.clone().into_inner();
            let transparency = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let part_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Tran {
                transparency,
                part_no,
            })
        }
        Rule::command_addb => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let background_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Addb {
                metaroom_id,
                background_file,
            })
        }
        Rule::command_altr => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let ca_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let ca_delta = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Altr {
                room_id,
                ca_index,
                ca_delta,
            })
        }
        Rule::command_calc => {
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
            let ca_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Cacl {
                family,
                genus,
                species,
                ca_index,
            })
        }
        Rule::command_delm => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Delm { metaroom_id })
        }
        Rule::command_delr => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Delr { room_id })
        }
        Rule::command_dmap => {
            let mut it = pair.clone().into_inner();
            let debug_map = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Dmap { debug_map })
        }
        Rule::command_doca => {
            let mut it = pair.clone().into_inner();
            let no_of_updates = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Doca { no_of_updates })
        }
        Rule::command_door => {
            let mut it = pair.clone().into_inner();
            let room_id1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let room_id2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let permiability = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Door {
                room_id1,
                room_id2,
                permiability,
            })
        }
        Rule::command_emit => {
            let mut it = pair.clone().into_inner();
            let ca_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let amount = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Emit { ca_index, amount })
        }
        Rule::command_link => {
            let mut it = pair.clone().into_inner();
            let room1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let room2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let permiability = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Link {
                room1,
                room2,
                permiability,
            })
        }
        Rule::command_mapd => {
            let mut it = pair.clone().into_inner();
            let width = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let height = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Mapd { width, height })
        }
        Rule::command_mapk => Ok(Command::Mapk),
        Rule::command_perm => {
            let mut it = pair.clone().into_inner();
            let permiability = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Perm { permiability })
        }
        Rule::command_prop => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let ca_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let ca_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Prop {
                room_id,
                ca_index,
                ca_value,
            })
        }
        Rule::command_rate => {
            let mut it = pair.clone().into_inner();
            let room_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let ca_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let gain = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let loss = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let diffusion = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Rate {
                room_type,
                ca_index,
                gain,
                loss,
                diffusion,
            })
        }
        Rule::command_rtyp => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let room_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Rtyp { room_id, room_type })
        }
        Rule::command_accg => {
            let mut it = pair.clone().into_inner();
            let acceleration = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Accg { acceleration })
        }
        Rule::command_aero => {
            let mut it = pair.clone().into_inner();
            let aerodynamics = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Aero { aerodynamics })
        }
        Rule::command_elas => {
            let mut it = pair.clone().into_inner();
            let elasticity = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Elas { elasticity })
        }
        Rule::command_flto => {
            let mut it = pair.clone().into_inner();
            let screen_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let screen_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Flto { screen_x, screen_y })
        }
        Rule::command_frel => {
            let mut it = pair.clone().into_inner();
            let relative = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::Frel { relative })
        }
        Rule::command_fric => {
            let mut it = pair.clone().into_inner();
            let friction = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Fric { friction })
        }
        Rule::command_mvby => {
            let mut it = pair.clone().into_inner();
            let delta_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let delta_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Mvby { delta_x, delta_y })
        }
        Rule::command_mvsf => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Mvsf { x, y })
        }
        Rule::command_mvto => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Mvto { x, y })
        }
        Rule::command_velo => {
            let mut it = pair.clone().into_inner();
            let x_velocity = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            let y_velocity = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)?;
            Ok(Command::Velo {
                x_velocity,
                y_velocity,
            })
        }
        Rule::command_prt_bang => {
            let mut it = pair.clone().into_inner();
            let bang_strength = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PrtBang { bang_strength })
        }
        Rule::command_prt_inew => {
            let mut it = pair.clone().into_inner();
            let id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let description = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let message_num = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PrtInew {
                id,
                name,
                description,
                x,
                y,
                message_num,
            })
        }
        Rule::command_prt_izap => {
            let mut it = pair.clone().into_inner();
            let id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PrtIzap { id })
        }
        Rule::command_prt_join => {
            let mut it = pair.clone().into_inner();
            let source_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let output_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let dest_agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let input_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PrtJoin {
                source_agent,
                output_id,
                dest_agent,
                input_id,
            })
        }
        Rule::command_prt_krak => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let in_or_out = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let port_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PrtKrak {
                agent,
                in_or_out,
                port_index,
            })
        }
        Rule::command_prt_onew => {
            let mut it = pair.clone().into_inner();
            let id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let description = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PrtOnew {
                id,
                name,
                description,
                x,
                y,
            })
        }
        Rule::command_prt_ozap => {
            let mut it = pair.clone().into_inner();
            let id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PrtOzap { id })
        }
        Rule::command_prt_send => {
            let mut it = pair.clone().into_inner();
            let id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let data = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_anything)?;
            Ok(Command::PrtSend { id, data })
        }
        Rule::command_pray_garb => {
            let mut it = pair.clone().into_inner();
            let force = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::PrayGarb { force })
        }
        Rule::command_pray_refr => Ok(Command::PrayRefr),
        Rule::command_gids_fmly => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::GidsFmly { family })
        }
        Rule::command_gids_gnus => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::GidsGnus { family, genus })
        }
        Rule::command_gids_root => Ok(Command::GidsRoot),
        Rule::command_gids_spec => {
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
            Ok(Command::GidsSpcs {
                family,
                genus,
                species,
            })
        }
        Rule::command_inst => Ok(Command::Inst),
        Rule::command_lock => Ok(Command::Lock),
        Rule::command_scrx => {
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
            let event = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Scrx {
                family,
                genus,
                species,
                event,
            })
        }
        Rule::command_slow => Ok(Command::Slow),
        Rule::command_sorc => {
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
            let event = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Sorc {
                family,
                genus,
                species,
                event,
            })
        }
        Rule::command_stop => Ok(Command::Stop),
        Rule::command_stpt => Ok(Command::Stpt),
        Rule::command_unlk => Ok(Command::Unlk),
        Rule::command_wait => {
            let mut it = pair.clone().into_inner();
            let ticks = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Wait { ticks })
        }
        Rule::command_fade => Ok(Command::Fade),
        Rule::command_mclr => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Mclr { x, y })
        }
        Rule::command_midi => {
            let mut it = pair.clone().into_inner();
            let midi_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Midi { midi_file })
        }
        Rule::command_mmsc => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let track_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Mmsc { x, y, track_name })
        }
        Rule::command_rclr => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Rclr { x, y })
        }
        Rule::command_rmsc => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let track_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Rmsc { x, y, track_name })
        }
        Rule::command_sezz => {
            let mut it = pair.clone().into_inner();
            let text = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Sezz { text })
        }
        Rule::command_sndc => {
            let mut it = pair.clone().into_inner();
            let sound_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Sndc { sound_file })
        }
        Rule::command_snde => {
            let mut it = pair.clone().into_inner();
            let sound_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Snde { sound_file })
        }
        Rule::command_sndl => {
            let mut it = pair.clone().into_inner();
            let sound_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Sndl { sound_file })
        }
        Rule::command_sndq => {
            let mut it = pair.clone().into_inner();
            let sound_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            let delay = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Sndq { sound_file, delay })
        }
        Rule::command_stpc => Ok(Command::Stpc),
        Rule::command_strk => {
            let mut it = pair.clone().into_inner();
            let latency = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let track = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Strk { latency, track })
        }
        Rule::command_voic => {
            let mut it = pair.clone().into_inner();
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let gender = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let age = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Voic { genus, gender, age })
        }
        Rule::command_vois => {
            let mut it = pair.clone().into_inner();
            let voice_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Vois { voice_name })
        }
        Rule::command_volm => {
            let mut it = pair.clone().into_inner();
            let volume = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Volm { volume })
        }
        Rule::command_wpau => {
            let mut it = pair.clone().into_inner();
            let paused = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Wpau { paused })
        }
        Rule::command_absv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            Ok(Command::Absv { var })
        }
        Rule::command_adds => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let append = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Adds { var, append })
        }
        Rule::command_addv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let sum = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            Ok(Command::Addv { var, sum })
        }
        Rule::command_andv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Andv { var, value })
        }
        Rule::command_char => {
            let mut it = pair.clone().into_inner();
            let string = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let character = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Char {
                string,
                index,
                character,
            })
        }
        Rule::command_delg => {
            let mut it = pair.clone().into_inner();
            let variable_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Delg { variable_name })
        }
        Rule::command_divv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let div = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            Ok(Command::Divv { var, div })
        }
        Rule::command_modv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let r#mod = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Modv { var, r#mod })
        }
        Rule::command_mulv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let mul = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            Ok(Command::Mulv { var, mul })
        }
        Rule::command_negv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            Ok(Command::Negv { var })
        }
        Rule::command_orrv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Orrv { var, value })
        }
        Rule::command_reaf => Ok(Command::Reaf),
        Rule::command_seta => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::Seta { var, value })
        }
        Rule::command_sets => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Sets { var, value })
        }
        Rule::command_setv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            Ok(Command::Setv { var, value })
        }
        Rule::command_subv => {
            let mut it = pair.clone().into_inner();
            let var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)?;
            let sub = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)?;
            Ok(Command::Subv { var, sub })
        }
        Rule::command_targ => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::Targ { agent })
        }
        Rule::command_cabn => {
            let mut it = pair.clone().into_inner();
            let left = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let top: crate::ast::IntArg = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let right = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            let bottom = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Cabn {
                left,
                top,
                right,
                bottom,
            })
        }
        Rule::command_cabp => {
            let mut it = pair.clone().into_inner();
            let plane = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Cabp { plane })
        }
        Rule::command_cabv => {
            let mut it = pair.clone().into_inner();
            let cabin_room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Cabv { cabin_room_id })
        }
        Rule::command_cabw => {
            let mut it = pair.clone().into_inner();
            let cabin_capacity = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Cabw { cabin_capacity })
        }
        Rule::command_dpas => {
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
            Ok(Command::Dpas {
                family,
                genus,
                species,
            })
        }
        Rule::command_gpas => {
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
            let rect_to_use = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Gpas {
                family,
                genus,
                species,
                rect_to_use,
            })
        }
        Rule::command_new_vhcl => {
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
            Ok(Command::NewVhcl {
                family,
                genus,
                species,
                sprite_file,
                image_count,
                first_image,
                plane,
            })
        }
        Rule::command_rpas => {
            let mut it = pair.clone().into_inner();
            let vehicle = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let passenger = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::Rpas { vehicle, passenger })
        }
        Rule::command_spas => {
            let mut it = pair.clone().into_inner();
            let vehicle = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            let new_passenger = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)?;
            Ok(Command::Spas {
                vehicle,
                new_passenger,
            })
        }
        Rule::command_delw => {
            let mut it = pair.clone().into_inner();
            let world_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Delw { world_name })
        }
        Rule::command_load => {
            let mut it = pair.clone().into_inner();
            let world_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Load { world_name })
        }
        Rule::command_pswd => {
            let mut it = pair.clone().into_inner();
            let world_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Pswd { world_name })
        }
        Rule::command_quit => Ok(Command::Quit),
        Rule::command_rgam => Ok(Command::Rgam),
        Rule::command_save => Ok(Command::Save),
        Rule::command_tntw => {
            let mut it = pair.clone().into_inner();
            let index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
            Ok(Command::Tntw { index })
        }
        Rule::command_wrld => {
            let mut it = pair.clone().into_inner();
            let world_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)?;
            Ok(Command::Wrld { world_name })
        }
        Rule::command_wtnt => {
            let mut it = pair.clone().into_inner();
            let index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)?;
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
            Ok(Command::Wtnt {
                index,
                red_tint,
                green_tint,
                blue_tint,
                rotation,
                swap,
            })
        }
        _ => todo!(),
    }
}
