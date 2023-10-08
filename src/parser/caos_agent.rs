use super::parse_variable;
use crate::{ast::Agent, ast::AgentArg, parse_int_arg, parse_string_arg, CaosError, Rule};
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

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Agent, AgentArg, IntArg, Integer, SString, SStringArg, Variable},
        parser::CaosParser,
    };
    use pest::Parser;

    use super::*;

    #[test]
    fn test_agent_arg() {
        for p in CaosParser::parse(Rule::agent_arg, "MV00").expect("Parsed") {
            assert_eq!(
                parse_agent_arg(p).expect("Parsed variable"),
                AgentArg::Variable(Variable::Mvxx(0)),
            );
        }
        for p in CaosParser::parse(Rule::agent_arg, "PNTR").expect("Parsed") {
            assert_eq!(
                parse_agent_arg(p).expect("Parsed variable"),
                AgentArg::Agent(Agent::Pntr),
            );
        }
    }

    #[test]
    fn test_agent_arg_fail() {
        CaosParser::parse(Rule::agent_arg, "2").expect_err("Can't parse int as agent");
    }

    #[test]
    fn test_agent_carr() {
        for p in CaosParser::parse(Rule::agent, "CARR").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Carr,);
        }
    }

    #[test]
    fn test_agent_from() {
        for p in CaosParser::parse(Rule::agent, "FRoM").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::From,);
        }
    }

    #[test]
    fn test_agent_held() {
        for p in CaosParser::parse(Rule::agent, "held").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Held,);
        }
    }

    #[test]
    fn test_agent_iitt() {
        for p in CaosParser::parse(Rule::agent, "iitt").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Iitt,);
        }
    }

    #[test]
    fn test_agent_ncls() {
        for p in CaosParser::parse(Rule::agent, "NCLS MV00 0 100 %10").expect("Parsed") {
            assert_eq!(
                parse_agent(p).expect("Parsed variable"),
                Agent::Ncls {
                    previous: Box::new(AgentArg::Variable(Variable::Mvxx(0))),
                    family: Box::new(IntArg::Primary(Integer::Literal(0))),
                    genus: Box::new(IntArg::Primary(Integer::Literal(100))),
                    species: Box::new(IntArg::Primary(Integer::Literal(2)))
                },
            );
        }
    }

    #[test]
    fn test_agent_null() {
        for p in CaosParser::parse(Rule::agent, "NULL").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Null,);
        }
    }

    #[test]
    fn test_agent_ownr() {
        for p in CaosParser::parse(Rule::agent, "ownr").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Ownr,);
        }
    }

    #[test]
    fn test_agent_pcls() {
        for p in CaosParser::parse(Rule::agent, "PCLS _it_ 500 500 500").expect("Parsed") {
            assert_eq!(
                parse_agent(p).expect("Parsed variable"),
                Agent::Pcls {
                    next: Box::new(AgentArg::Agent(Agent::It)),
                    family: Box::new(IntArg::Primary(Integer::Literal(500))),
                    genus: Box::new(IntArg::Primary(Integer::Literal(500))),
                    species: Box::new(IntArg::Primary(Integer::Literal(500)))
                }
            );
        }
    }

    #[test]
    fn test_agent_pntr() {
        for p in CaosParser::parse(Rule::agent, "pntr").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Pntr,);
        }
    }

    #[test]
    fn test_agent_targ() {
        for p in CaosParser::parse(Rule::agent, "targ").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Targ,);
        }
    }

    #[test]
    fn test_agent_twin() {
        for p in CaosParser::parse(Rule::agent, "TWIN TARG 'A'").expect("Parsed") {
            assert_eq!(
                parse_agent(p).expect("Parsed variable"),
                Agent::Twin {
                    original: Box::new(AgentArg::Agent(Agent::Targ)),
                    agent_null: Box::new(IntArg::Primary(Integer::Literal('A' as i32))),
                }
            );
        }
    }

    #[test]
    fn test_agent_it() {
        for p in CaosParser::parse(Rule::agent, "_IT_").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::It,);
        }
    }

    #[test]
    fn test_agent_trck() {
        for p in CaosParser::parse(Rule::agent, "TRCK").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Trck,);
        }
    }

    #[test]
    fn test_agent_hhld() {
        for p in CaosParser::parse(Rule::agent, "Hhld").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Hhld,);
        }
    }

    #[test]
    fn test_agent_norn() {
        for p in CaosParser::parse(Rule::agent, "NORN").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Norn,);
        }
    }

    #[test]
    fn test_agent_agnt() {
        for p in CaosParser::parse(Rule::agent, "AGNT 0").expect("Parsed") {
            assert_eq!(
                parse_agent(p).expect("Parsed variable"),
                Agent::Agnt {
                    unique_id: Box::new(IntArg::Primary(Integer::Literal(0)))
                },
            );
        }
    }

    #[test]
    fn test_agent_tack() {
        for p in CaosParser::parse(Rule::agent, "TACK").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Tack,);
        }
    }

    #[test]
    fn test_agent_mtoa() {
        for p in CaosParser::parse(Rule::agent, r#"MTOA "FOO""#).expect("Parsed") {
            assert_eq!(
                parse_agent(p).expect("Parsed variable"),
                Agent::Mtoa {
                    moniker: Box::new(SStringArg::String(SString::Literal("FOO".to_owned())))
                },
            );
        }
    }

    #[test]
    fn test_agent_mtoc() {
        for p in CaosParser::parse(Rule::agent, r#"MTOC "FOO""#).expect("Parsed") {
            assert_eq!(
                parse_agent(p).expect("Parsed variable"),
                Agent::Mtoc {
                    moniker: Box::new(SStringArg::String(SString::Literal("FOO".to_owned())))
                },
            );
        }
    }

    #[test]
    fn test_agent_hots() {
        for p in CaosParser::parse(Rule::agent, "HOTS").expect("Parsed") {
            assert_eq!(parse_agent(p).expect("Parsed variable"), Agent::Hots,);
        }
    }

    #[test]
    fn test_agent_ptr_frma() {
        for p in CaosParser::parse(Rule::agent, "PRT: FRMA 4").expect("Parsed") {
            assert_eq!(
                parse_agent(p).expect("Parsed variable"),
                Agent::PrtFrma {
                    input_port: Box::new(IntArg::Primary(Integer::Literal(4)))
                },
            );
        }
    }
}
