#[cfg(test)]
mod tests;

use super::{parse_int_arg, parse_string_arg, parse_variable};
use crate::{ast::Agent, ast::AgentArg, CaosError, Rule};
use pest::iterators::Pair;

pub fn parse_agent(pair: Pair<Rule>) -> Result<Agent, CaosError> {
    if pair.as_rule() != Rule::agent {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::agent_carr => Ok(Agent::Carr),
        Rule::agent_from => Ok(Agent::From),
        Rule::agent_held => Ok(Agent::Held),
        Rule::agent_iitt => Ok(Agent::Iitt),
        Rule::agent_ncls => {
            let mut it = pair.clone().into_inner();
            let previous = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
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
                .ok_or(CaosError::new_parse_error(pair))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Agent::Ncls {
                previous,
                family,
                genus,
                species,
            })
        }
        Rule::agent_null => Ok(Agent::Null),
        Rule::agent_ownr => Ok(Agent::Ownr),
        Rule::agent_pcls => {
            let mut it = pair.clone().into_inner();
            let next = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
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
                .ok_or(CaosError::new_parse_error(pair))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Agent::Pcls {
                next,
                family,
                genus,
                species,
            })
        }
        Rule::agent_pntr => Ok(Agent::Pntr),
        Rule::agent_targ => Ok(Agent::Targ),
        Rule::agent_twin => {
            let mut it = pair.clone().into_inner();
            let original = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_agent_arg)
                .map(Box::new)?;
            let agent_null = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Agent::Twin {
                original,
                agent_null,
            })
        }
        Rule::agent_it => Ok(Agent::It),
        Rule::agent_trck => Ok(Agent::Trck),
        Rule::agent_hhld => Ok(Agent::Hhld),
        Rule::agent_norn => Ok(Agent::Norn),
        Rule::agent_agnt => {
            let mut it = pair.clone().into_inner();
            let unique_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Agent::Agnt { unique_id })
        }
        Rule::agent_tack => Ok(Agent::Tack),
        Rule::agent_mtoa => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Agent::Mtoa { moniker })
        }
        Rule::agent_mtoc => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(Agent::Mtoc { moniker })
        }
        Rule::agent_hots => Ok(Agent::Hots),
        Rule::agent_prt_frma => {
            let mut it = pair.clone().into_inner();
            let input_port = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(Agent::PrtFrma { input_port })
        }
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_agent_arg(pair: Pair<Rule>) -> Result<AgentArg, CaosError> {
    match pair.as_rule() {
        Rule::agent => parse_agent(pair).map(AgentArg::Agent),
        Rule::variable => parse_variable(pair).map(AgentArg::Variable),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}
