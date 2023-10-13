#[cfg(test)]
mod tests;

use super::{
    parse_agent_arg, parse_anything, parse_decimal_arg, parse_int_arg, parse_string_literal,
    parse_variable,
};
use crate::{ast::SString, ast::SStringArg, CaosError, Rule};
use pest::iterators::Pair;

pub fn parse_string(pair: Pair<Rule>) -> Result<SString, CaosError> {
    if pair.as_rule() != Rule::string {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::literal_string => parse_string_literal(pair).map(SString::Literal),
        Rule::string_catx => {
            let mut it = pair.clone().into_inner();
            let category_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Catx { category_id })
        }
        Rule::string_hand => Ok(SString::Hand),
        Rule::string_wild => {
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
            let tag_stub = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let offset = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Wild {
                family,
                genus,
                species,
                tag_stub,
                offset,
            })
        }
        Rule::string_bkgd => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Bkgd { metaroom_id })
        }
        Rule::string_ptxt => Ok(SString::Ptxt),
        Rule::string_face => Ok(SString::Face),
        Rule::string_dbg => {
            let mut it = pair.clone().into_inner();
            let variable = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Dbg { variable })
        }
        Rule::string_dbga => {
            let mut it = pair.clone().into_inner();
            let variable = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Dbga { variable })
        }
        Rule::string_fvwm => {
            let mut it = pair.clone().into_inner();
            let name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::Fvwm { name })
        }
        Rule::string_innl => Ok(SString::Innl),
        Rule::string_gtos => {
            let mut it = pair.clone().into_inner();
            let slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Gtos { slot })
        }
        Rule::string_hist_foto => {
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
            Ok(SString::HistFoto { moniker, event_no })
        }
        Rule::string_hist_mon1 => {
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
            Ok(SString::HistMon1 { moniker, event_no })
        }
        Rule::string_hist_mon2 => {
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
            Ok(SString::HistMon2 { moniker, event_no })
        }
        Rule::string_hist_name => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::HistName { moniker })
        }
        Rule::string_hist_next => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::HistNext { moniker })
        }
        Rule::string_hist_prev => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::HistPrev { moniker })
        }
        Rule::string_hist_utxt => {
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
            Ok(SString::HistUtxt { moniker, event_no })
        }
        Rule::string_hist_wnam => {
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
            Ok(SString::HistWnam { moniker, event_no })
        }
        Rule::string_bkds => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Bkds { metaroom_id })
        }
        Rule::string_emid => Ok(SString::Emid),
        Rule::string_erid => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Erid { metaroom_id })
        }
        Rule::string_mloc => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Mloc { metaroom_id })
        }
        Rule::string_rate => {
            let mut it = pair.clone().into_inner();
            let room_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let ca_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Rate {
                room_type,
                ca_index,
            })
        }
        Rule::string_rloc => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Rloc { room_id })
        }
        Rule::string_pray_agts => {
            let mut it = pair.clone().into_inner();
            let resource_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let string_tag = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let default_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::PrayAgts {
                resource_name,
                string_tag,
                default_value,
            })
        }
        Rule::string_pray_next => {
            let mut it = pair.clone().into_inner();
            let resource_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let last_known = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::PrayNext {
                resource_type,
                last_known,
            })
        }
        Rule::string_pray_prev => {
            let mut it = pair.clone().into_inner();
            let resource_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let last_known = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::PrayPrev {
                resource_type,
                last_known,
            })
        }
        Rule::string_caos => {
            let mut it = pair.clone().into_inner();
            let inline = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let state_trans = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let p1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_anything)
                .map(Box::new)?;
            let p2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_anything)
                .map(Box::new)?;
            let commands = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let throws = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let catches = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let report = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)
                .map(Box::new)?;
            Ok(SString::Caos {
                inline,
                state_trans,
                p1,
                p2,
                commands,
                throws,
                catches,
                report,
            })
        }
        Rule::string_rmsc => {
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
            Ok(SString::Rmsc { x, y })
        }
        Rule::string_vois => Ok(SString::Vois),
        Rule::string_rtif => {
            let mut it = pair.clone().into_inner();
            let real_time = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let format = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::Rtif { real_time, format })
        }
        Rule::string_gamn => {
            let mut it = pair.clone().into_inner();
            let previous = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::Gamn { previous })
        }
        Rule::string_gnam => Ok(SString::Gnam),
        Rule::string_read => {
            let mut it = pair.clone().into_inner();
            let catalogue_tag = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let offset = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Read {
                catalogue_tag,
                offset,
            })
        }
        Rule::string_subs => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let start = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let count = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Subs {
                value,
                start,
                count,
            })
        }
        Rule::string_vtos => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)
                .map(Box::new)?;
            Ok(SString::Vtos { value })
        }
        Rule::string_pswd => {
            let mut it = pair.clone().into_inner();
            let world_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Pswd { world_index })
        }
        Rule::string_wnam => Ok(SString::Wnam),
        Rule::string_wrld => {
            let mut it = pair.clone().into_inner();
            let world_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Wrld { world_index })
        }
        Rule::string_wuid => Ok(SString::Wuid),
        Rule::string_prt_name => {
            let mut it = pair.clone().into_inner();
            let agent = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            let in_or_out = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let port_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::PrtName {
                agent,
                in_or_out,
                port_index,
            })
        }
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_string_arg(pair: Pair<Rule>) -> Result<SStringArg, CaosError> {
    match pair.as_rule() {
        Rule::string => parse_string(pair).map(SStringArg::String),
        Rule::variable => parse_variable(pair).map(SStringArg::Variable),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}
