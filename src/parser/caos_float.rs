#[cfg(test)]
mod tests;

use super::{
    parse_agent_arg, parse_float_literal, parse_int, parse_int_arg, parse_string_arg,
    parse_variable,
};
use crate::{ast::Float, ast::FloatArg, CaosError, Rule};
use pest::iterators::Pair;

pub fn parse_float(pair: Pair<Rule>) -> Result<Float, CaosError> {
    if pair.as_rule() != Rule::float {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::literal_float => parse_float_literal(pair).map(|f| Float::Literal(f.into())),
        Rule::float_disq => {
            let mut it = pair.clone().into_inner();
            let other = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            Ok(Float::Disq { other })
        }
        Rule::float_fltx => Ok(Float::Fltx),
        Rule::float_flty => Ok(Float::Flty),
        Rule::float_mthx => Ok(Float::Mthx),
        Rule::float_mthy => Ok(Float::Mthy),
        Rule::float_posb => Ok(Float::Posb),
        Rule::float_posl => Ok(Float::Posl),
        Rule::float_posr => Ok(Float::Posr),
        Rule::float_post => Ok(Float::Post),
        Rule::float_posx => Ok(Float::Posx),
        Rule::float_posy => Ok(Float::Posy),
        Rule::float_rnge => Ok(Float::Rnge),
        Rule::float_chem => {
            let mut it = pair.clone().into_inner();
            let chemical = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Float::Chem { chemical })
        }
        Rule::float_dftx => Ok(Float::Dftx),
        Rule::float_dfty => Ok(Float::Dfty),
        Rule::float_driv => {
            let mut it = pair.clone().into_inner();
            let drive = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Float::Driv { drive })
        }
        Rule::float_loci => {
            let mut it = pair.clone().into_inner();
            let r#type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let organ = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let tissue = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Float::Loci {
                r#type,
                organ,
                tissue,
                id,
            })
        }
        Rule::float_orgf => {
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
            Ok(Float::Orgf { organ_number, data })
        }
        Rule::float_uftx => Ok(Float::Uftx),
        Rule::float_ufty => Ok(Float::Ufty),
        Rule::float_innf => Ok(Float::Innf),
        Rule::float_movx => Ok(Float::Movx),
        Rule::float_movy => Ok(Float::Movy),
        Rule::float_prop => {
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
            Ok(Float::Prop { room_id, ca_index })
        }
        Rule::float_torx => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Float::Torx { room_id })
        }
        Rule::float_tory => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Float::Tory { room_id })
        }
        Rule::float_accg => Ok(Float::Accg),
        Rule::float_obst => {
            let mut it = pair.clone().into_inner();
            let direction = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Float::Obst { direction })
        }
        Rule::float_relx => {
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
            Ok(Float::Relx { first, second })
        }
        Rule::float_rely => {
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
            Ok(Float::Rely { first, second })
        }
        Rule::float_pace => Ok(Float::Pace),
        Rule::float_acos => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Float::Acos { x })
        }
        Rule::float_asin => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Float::Asin { x })
        }
        Rule::float_atan => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Float::Atan { x })
        }
        Rule::float_cos => {
            let mut it = pair.clone().into_inner();
            let theta = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Float::Cos { theta })
        }
        Rule::float_itof => {
            let mut it = pair.clone().into_inner();
            let number_to_convert = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Float::Itof { number_to_convert })
        }
        Rule::float_sin => {
            let mut it = pair.clone().into_inner();
            let theta = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Float::Sin { theta })
        }
        Rule::float_sqrt => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Float::Sqrt { value })
        }
        Rule::float_stof => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Float::Stof { value })
        }
        Rule::float_tan => {
            let mut it = pair.clone().into_inner();
            let theta = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_float_arg)
                .map(Box::new)?;
            Ok(Float::Tan { theta })
        }
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_float_arg(pair: Pair<Rule>) -> Result<FloatArg, CaosError> {
    match pair.as_rule() {
        Rule::float => parse_float(pair).map(FloatArg::Primary),
        Rule::int => parse_int(pair).map(FloatArg::Castable),
        Rule::variable => parse_variable(pair).map(FloatArg::Variable),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}
