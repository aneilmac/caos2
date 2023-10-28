use super::parse_expr;
use crate::ast::{Agent, AgentArg, Variable};

fn parse_agent_arg(content: &str) -> AgentArg {
    parse_expr::<AgentArg>(content)
}

fn parse_agent(content: &str) -> Agent {
    let a = parse_agent_arg(content);
    match a {
        AgentArg::Agent(a) => a,
        AgentArg::Variable(_) => panic!("Expected Agent"),
    }
}

#[test]
fn test_agent_arg() {
    let p = parse_agent_arg("MV00");
    assert_eq!(p, Variable::Mvxx(0).into());

    let p = parse_agent_arg("PNTR");
    assert_eq!(p, Agent::Pntr.into());
}

#[test]
fn test_agent_carr() {
    let p = parse_agent("CARR");
    assert_eq!(p, Agent::Carr);
}

#[test]
fn test_agent_from() {
    let p = parse_agent("FROM");
    assert_eq!(p, Agent::From);
}

#[test]
fn test_agent_held() {
    let p = parse_agent("HELD");
    assert_eq!(p, Agent::Held);
}

#[test]
fn test_agent_iitt() {
    let p = parse_agent("IITT");
    assert_eq!(p, Agent::Iitt);
}

#[test]
fn test_agent_ncls() {
    let p = parse_agent("NCLS MV00 0 100 %10");
    assert_eq!(
        p,
        Agent::Ncls {
            previous: Box::new(Variable::Mvxx(0).into()),
            family: Box::new(0.into()),
            genus: Box::new(100.into()),
            species: Box::new(2.into()),
        }
    );
}

#[test]
fn test_agent_null() {
    let p = parse_agent("NULL");
    assert_eq!(p, Agent::Null);
}

#[test]
fn test_agent_ownr() {
    let p = parse_agent("OWNR");
    assert_eq!(p, Agent::Ownr);
}

#[test]
fn test_agent_pcls() {
    let p = parse_agent("PCLS _IT_ 500 600 700");
    assert_eq!(
        p,
        Agent::Pcls {
            next: Box::new(Agent::It.into()),
            family: Box::new(500.into()),
            genus: Box::new(600.into()),
            species: Box::new(700.into())
        }
    );
}

#[test]
fn test_agent_pntr() {
    let p = parse_agent("PNTR");
    assert_eq!(p, Agent::Pntr);
}

#[test]
fn test_agent_targ() {
    let p = parse_agent("TARG");
    assert_eq!(p, Agent::Targ);
}

#[test]
fn test_agent_twin() {
    let p = parse_agent("TWIN TARG 'A'");
    assert_eq!(
        p,
        Agent::Twin {
            original: Box::new(Agent::Targ.into()),
            agent_null: Box::new(('A' as i32).into()),
        }
    );
}

#[test]
fn test_agent_it() {
    let p = parse_agent("_IT_");
    assert_eq!(p, Agent::It);
}

#[test]
fn test_agent_trck() {
    let p = parse_agent("TRCK");
    assert_eq!(p, Agent::Trck);
}

#[test]
fn test_agent_hhld() {
    let p = parse_agent("HHLD");
    assert_eq!(p, Agent::Hhld);
}

#[test]
fn test_agent_norn() {
    let p = parse_agent("NORN");
    assert_eq!(p, Agent::Norn);
}

#[test]
fn test_agent_agnt() {
    let p = parse_agent("AGNT 0");
    assert_eq!(
        p,
        Agent::Agnt {
            unique_id: Box::new(0.into())
        }
    );
}

#[test]
fn test_agent_tack() {
    let p = parse_agent("TACK");
    assert_eq!(p, Agent::Tack);
}

#[test]
fn test_agent_mtoa() {
    let p = parse_agent(r#"MTOA "FOO""#);
    assert_eq!(
        p,
        Agent::Mtoa {
            moniker: Box::new(String::from("FOO").into())
        }
    );
}

#[test]
fn test_agent_mtoc() {
    let p = parse_agent(r#"MTOC "FOO""#);
    assert_eq!(
        p,
        Agent::Mtoc {
            moniker: Box::new(String::from("FOO").into())
        }
    );
}

#[test]
fn test_agent_hots() {
    let p = parse_agent("HOTS");
    assert_eq!(p, Agent::Hots);
}

#[test]
fn test_agent_ptr_frma() {
    let p = parse_agent("PRT: FRMA 4");
    assert_eq!(
        p,
        Agent::PrtFrma {
            input_port: Box::new(4.into())
        }
    );
}
