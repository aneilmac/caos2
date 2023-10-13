#[cfg(test)]
mod tests;

use super::{
    parse_agent_arg, parse_anything, parse_bytestring_literal, parse_float, parse_float_arg,
    parse_int_literal, parse_string_arg, parse_variable,
};
use crate::{ast::IntArg, ast::Integer, CaosError, Rule};
use pest::iterators::Pair;

pub fn parse_int(pair: Pair<Rule>) -> Result<Integer, CaosError> {
    if pair.as_rule() != Rule::int {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::literal_int => parse_int_literal(pair).map(Integer::Literal),
        Rule::int_attr => Ok(Integer::Attr),
        Rule::int_base => Ok(Integer::Base),
        Rule::int_bhvr => Ok(Integer::Bhvr),
        Rule::int_cati => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let species = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Cati {
                family,
                genus,
                species,
            })
        }
        Rule::int_clac => Ok(Integer::Clac),
        Rule::int_clik => {
            let mut it = pair.clone().into_inner();
            let which_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Clik { which_value })
        }
        Rule::int_fmly => Ok(Integer::Fmly),
        Rule::int_gnus => Ok(Integer::Gnus),
        Rule::int_hght => Ok(Integer::Hght),
        Rule::int_imsk => Ok(Integer::Imsk),
        Rule::int_mira => Ok(Integer::Mira),
        Rule::int_mows => Ok(Integer::Mows),
        Rule::int_paus => Ok(Integer::Paus),
        Rule::int_plne => Ok(Integer::Plne),
        Rule::int_pose => Ok(Integer::Pose),
        Rule::int_puhl => {
            let mut it = pair.clone().into_inner();
            let pose = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let x_or_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Puhl { pose, x_or_y })
        }
        Rule::int_pupt => {
            let mut it = pair.clone().into_inner();
            let pose = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let x_or_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Pupt { pose, x_or_y })
        }
        Rule::int_seee => {
            let mut it = pair.clone().into_inner();
            let first = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            let second = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            Ok(Integer::Seee { first, second })
        }
        Rule::int_spcs => Ok(Integer::Spcs),
        Rule::int_tick => Ok(Integer::Tick),
        Rule::int_totl => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let species = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Totl {
                family,
                genus,
                species,
            })
        }
        Rule::int_touc => {
            let mut it = pair.clone().into_inner();
            let first = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            let second = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            Ok(Integer::Touc { first, second })
        }
        Rule::int_visi => {
            let mut it = pair.clone().into_inner();
            let check_all_cameras = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Visi { check_all_cameras })
        }
        Rule::int_wdth => Ok(Integer::Wdth),
        Rule::int_cmrx => Ok(Integer::Cmrx),
        Rule::int_cmry => Ok(Integer::Cmry),
        Rule::int_loft => {
            let mut it = pair.clone().into_inner();
            let filename = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::Loft { filename })
        }
        Rule::int_meta => Ok(Integer::Meta),
        Rule::int_snax => {
            let mut it = pair.clone().into_inner();
            let filename = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::Snax { filename })
        }
        Rule::int_wdow => Ok(Integer::Wdow),
        Rule::int_wndb => Ok(Integer::Wndb),
        Rule::int_wndh => Ok(Integer::Wndh),
        Rule::int_wndl => Ok(Integer::Wndl),
        Rule::int_wndr => Ok(Integer::Wndr),
        Rule::int_wndt => Ok(Integer::Wndt),
        Rule::int_wndw => Ok(Integer::Wndw),
        Rule::int_npgs => Ok(Integer::Npgs),
        Rule::int_page => Ok(Integer::Page),
        Rule::int_aslp => Ok(Integer::Aslp),
        Rule::int_attn => Ok(Integer::Attn),
        Rule::int_body => {
            let mut it = pair.clone().into_inner();
            let body_part = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Body { body_part })
        }
        Rule::int_bvar => Ok(Integer::Bvar),
        Rule::int_byit => Ok(Integer::Byit),
        Rule::int_cage => Ok(Integer::Cage),
        Rule::int_crea => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            Ok(Integer::Crea { agent })
        }
        Rule::int_dead => Ok(Integer::Dead),
        Rule::int_decn => Ok(Integer::Decn),
        Rule::int_dirn => Ok(Integer::Dirn),
        Rule::int_drea => Ok(Integer::Drea),
        Rule::int_drv => Ok(Integer::Drv),
        Rule::int_expr => Ok(Integer::Expr),
        Rule::int_face => Ok(Integer::Face),
        Rule::int_ins => Ok(Integer::Ins),
        Rule::int_orgi => {
            let mut it = pair.clone().into_inner();
            let organ_number = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let data = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Orgi { organ_number, data })
        }
        Rule::int_orgn => Ok(Integer::Orgn),
        Rule::int_tage => Ok(Integer::Tage),
        Rule::int_uncs => Ok(Integer::Uncs),
        Rule::int_zomb => Ok(Integer::Zomb),
        Rule::int_code => Ok(Integer::Code),
        Rule::int_codf => Ok(Integer::Codf),
        Rule::int_codg => Ok(Integer::Codg),
        Rule::int_codp => Ok(Integer::Codp),
        Rule::int_cods => Ok(Integer::Cods),
        Rule::int_heap => {
            let mut it = pair.clone().into_inner();
            let index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Heap { index })
        }
        Rule::int_paws => Ok(Integer::Paws),
        Rule::int_unid => Ok(Integer::Unid),
        Rule::int_inni => Ok(Integer::Inni),
        Rule::int_inok => Ok(Integer::Inok),
        Rule::int_hist_cage => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistCage { moniker, event_no })
        }
        Rule::int_hist_coun => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::HistCoun { moniker })
        }
        Rule::int_hist_cros => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::HistCros { moniker })
        }
        Rule::int_hist_find => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let from_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistFind {
                moniker,
                event_type,
                from_index,
            })
        }
        Rule::int_hist_finr => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let from_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistFinr {
                moniker,
                event_type,
                from_index,
            })
        }
        Rule::int_hist_gend => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::HistGend { moniker })
        }
        Rule::int_hist_gnus => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::HistGnus { moniker })
        }
        Rule::int_hist_mute => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::HistMute { moniker })
        }
        Rule::int_hist_prev => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::HistPrev { moniker })
        }
        Rule::int_hist_rtim => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistRtim { moniker, event_no })
        }
        Rule::int_hist_tage => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistTage { moniker, event_no })
        }
        Rule::int_hist_type => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistType { moniker, event_no })
        }
        Rule::int_hist_vari => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::HistVari { moniker })
        }
        Rule::int_hist_wnam => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistWnam { moniker, event_no })
        }
        Rule::int_hist_wtik => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistWtik { moniker, event_no })
        }
        Rule::int_hist_wuid => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistWuid { moniker, event_no })
        }
        Rule::int_ooww => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::Ooww { moniker })
        }
        Rule::int_keyd => {
            let mut it = pair.clone().into_inner();
            let key_code = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Keyd { key_code })
        }
        Rule::int_mopx => Ok(Integer::Mopx),
        Rule::int_mopy => Ok(Integer::Mopy),
        Rule::int_pure => Ok(Integer::Pure),
        Rule::int_addm => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let width = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let height = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let background = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::Addm {
                x,
                y,
                width,
                height,
                background,
            })
        }
        Rule::int_addr => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let x_left = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let y_right = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let y_left_ceiling = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let y_right_ceiling = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let y_left_floor = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let y_right_floor = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Addr {
                metaroom_id,
                x_left,
                y_right,
                y_left_ceiling,
                y_right_ceiling,
                y_left_floor,
                y_right_floor,
            })
        }
        Rule::int_door => {
            let mut it = pair.clone().into_inner();
            let room_id1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let room_id2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Door { room_id1, room_id2 })
        }
        Rule::int_down => Ok(Integer::Down),
        Rule::int_gmap => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Integer::Gmap { x, y })
        }
        Rule::int_grap => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Integer::Grap { x, y })
        }
        Rule::int_grid => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            let direction = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Grid { agent, direction })
        }
        Rule::int_hirp => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let ca_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let directions = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Hirp {
                room_id,
                ca_index,
                directions,
            })
        }
        Rule::int_left => Ok(Integer::Left),
        Rule::int_link => {
            let mut it = pair.clone().into_inner();
            let room1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let room2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Link { room1, room2 })
        }
        Rule::int_lorp => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let ca_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let directions = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Lorp {
                room_id,
                ca_index,
                directions,
            })
        }
        Rule::int_maph => Ok(Integer::Maph),
        Rule::int_mapk => Ok(Integer::Mapk),
        Rule::int_mapw => Ok(Integer::Mapw),
        Rule::int_perm => Ok(Integer::Perm),
        Rule::int_rght => Ok(Integer::Rght),
        Rule::int_room => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            Ok(Integer::Room { agent })
        }
        Rule::int_rtyp => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Rtyp { room_id })
        }
        Rule::int_up => Ok(Integer::Up),
        Rule::int_aero => Ok(Integer::Aero),
        Rule::int_elas => Ok(Integer::Elas),
        Rule::int_fall => Ok(Integer::Fall),
        Rule::int_fric => Ok(Integer::Fric),
        Rule::int_movs => Ok(Integer::Movs),
        Rule::int_tmvb => {
            let mut it = pair.clone().into_inner();
            let delta_x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            let delta_y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Integer::Tmvb { delta_x, delta_y })
        }
        Rule::int_tmvf => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Integer::Tmvf { x, y })
        }
        Rule::int_tmvt => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Integer::Tmvt { x, y })
        }
        Rule::int_wall => Ok(Integer::Wall),
        Rule::int_pray_agti => {
            let mut it = pair.clone().into_inner();
            let resource_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let integer_tag = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let default_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::PrayAgti {
                resource_name,
                integer_tag,
                default_value,
            })
        }
        Rule::int_pray_coun => {
            let mut it = pair.clone().into_inner();
            let resource_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::PrayCoun { resource_type })
        }
        Rule::int_pray_deps => {
            let mut it = pair.clone().into_inner();
            let resource_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let dp_install = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::PrayDeps {
                resource_name,
                dp_install,
            })
        }
        Rule::int_pray_expo => {
            let mut it = pair.clone().into_inner();
            let chunk_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::PrayExpo { chunk_name })
        }
        Rule::int_pray_file => {
            let mut it = pair.clone().into_inner();
            let resource_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let resource_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let do_install = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::PrayFile {
                resource_name,
                resource_type,
                do_install,
            })
        }
        Rule::int_pray_impo => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let actually_do_it = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let keep_file = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::PrayImpo {
                moniker,
                actually_do_it,
                keep_file,
            })
        }
        Rule::int_pray_injt => {
            let mut it = pair.clone().into_inner();
            let resource_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let do_install = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let report_var = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)
                .map(Box::new)?;
            Ok(Integer::PrayInjt {
                resource_name,
                do_install,
                report_var,
            })
        }
        Rule::int_pray_make => {
            let mut it = pair.clone().into_inner();
            let which_journal_spot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let journal_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let which_pray_spot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let pray_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let report_destination = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)
                .map(Box::new)?;
            Ok(Integer::PrayMake {
                which_journal_spot,
                journal_name,
                which_pray_spot,
                pray_name,
                report_destination,
            })
        }
        Rule::int_pray_size => {
            let mut it = pair.clone().into_inner();
            let resource_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::PraySize { resource_name })
        }
        Rule::int_pray_test => {
            let mut it = pair.clone().into_inner();
            let resource_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::PrayTest { resource_name })
        }
        Rule::int_sorq => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let species = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let event = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Sorq {
                family,
                genus,
                species,
                event,
            })
        }
        Rule::int_mute => {
            let mut it = pair.clone().into_inner();
            let and_mask = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let eor_mask = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Mute { and_mask, eor_mask })
        }
        Rule::int_date => Ok(Integer::Date),
        Rule::int_dayt => Ok(Integer::Dayt),
        Rule::int_etik => Ok(Integer::Etik),
        Rule::int_hist_date => {
            let mut it = pair.clone().into_inner();
            let world_tick = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistDate { world_tick })
        }
        Rule::int_hist_sean => {
            let mut it = pair.clone().into_inner();
            let world_tick = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistSean { world_tick })
        }
        Rule::int_hist_time => {
            let mut it = pair.clone().into_inner();
            let world_tick = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistTime { world_tick })
        }
        Rule::int_hist_year => {
            let mut it = pair.clone().into_inner();
            let world_tick = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::HistYear { world_tick })
        }
        Rule::int_mont => Ok(Integer::Mont),
        Rule::int_msec => Ok(Integer::Msec),
        Rule::int_race => Ok(Integer::Race),
        Rule::int_rtim => Ok(Integer::Rtim),
        Rule::int_scol => {
            let mut it = pair.clone().into_inner();
            let and_mask = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let eor_mask = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let up_speeds = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_bytestring_literal)
                .map(Box::new)?;
            let down_speeds = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_bytestring_literal)
                .map(Box::new)?;
            Ok(Integer::Scol {
                and_mask,
                eor_mask,
                up_speeds,
                down_speeds,
            })
        }
        Rule::int_sean => Ok(Integer::Sean),
        Rule::int_time => Ok(Integer::Time),
        Rule::int_wolf => {
            let mut it = pair.clone().into_inner();
            let kanga_mask = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let eeyore_mask = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Wolf {
                kanga_mask,
                eeyore_mask,
            })
        }
        Rule::int_wpau => Ok(Integer::Wpau),
        Rule::int_wtik => Ok(Integer::Wtik),
        Rule::int_year => Ok(Integer::Year),
        Rule::int_char => {
            let mut it = pair.clone().into_inner();
            let string = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Char { string, index })
        }
        Rule::int_ftoi => {
            let mut it = pair.clone().into_inner();
            let number_to_convert = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Integer::Ftoi { number_to_convert })
        }
        Rule::int_rand => {
            let mut it = pair.clone().into_inner();
            let value1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let value2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::Rand { value1, value2 })
        }
        Rule::int_rean => {
            let mut it = pair.clone().into_inner();
            let catalogue_tag = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::Rean { catalogue_tag })
        }
        Rule::int_reaq => {
            let mut it = pair.clone().into_inner();
            let catalogue_tag = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::Reaq { catalogue_tag })
        }
        Rule::int_stoi => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::Stoi { value })
        }
        Rule::int_strl => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::Strl { value })
        }
        Rule::int_type => {
            let mut it = pair.clone().into_inner();
            let something = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_anything)
                .map(Box::new)?;
            Ok(Integer::Type { something })
        }
        Rule::int_vmjr => Ok(Integer::Vmjr),
        Rule::int_vmnr => Ok(Integer::Vmnr),
        Rule::int_cabb => Ok(Integer::Cabb),
        Rule::int_cabl => Ok(Integer::Cabl),
        Rule::int_cabp => Ok(Integer::Cabp),
        Rule::int_cabr => Ok(Integer::Cabr),
        Rule::int_cabt => Ok(Integer::Cabt),
        Rule::int_cabv => Ok(Integer::Cabv),
        Rule::int_nwld => Ok(Integer::Nwld),
        Rule::int_wnti => {
            let mut it = pair.clone().into_inner();
            let world = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Integer::Wnti { world })
        }
        Rule::int_prt_itot => Ok(Integer::PrtItot),
        Rule::int_prt_from => {
            let mut it = pair.clone().into_inner();
            let input_port = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Integer::PrtFrom { input_port })
        }
        Rule::int_prt_otot => Ok(Integer::PrtOtot),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_int_arg(pair: Pair<Rule>) -> Result<IntArg, CaosError> {
    match pair.as_rule() {
        Rule::float => parse_float(pair).map(IntArg::Castable),
        Rule::int => parse_int(pair).map(IntArg::Primary),
        Rule::variable => parse_variable(pair).map(IntArg::Variable),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}
