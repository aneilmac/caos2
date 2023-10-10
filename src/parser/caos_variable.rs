use crate::{ast::Variable, CaosError, Rule};
use pest::iterators::Pair;

fn parse_variable_digit(pair: Pair<Rule>) -> Result<u8, CaosError> {
    if pair.as_rule() != Rule::variable_digit {
        return Err(CaosError::new_parse_error(pair));
    }
    pair.as_str()
        .parse::<u8>()
        .map_err(|_| CaosError::new_parse_error(pair))
}

pub fn parse_variable(pair: Pair<Rule>) -> Result<Variable, CaosError> {
    if pair.as_rule() != Rule::variable {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::variable_mvxx => {
            let d = pair
                .clone()
                .into_inner()
                .next()
                .ok_or(CaosError::new_parse_error(pair))?;
            let d = parse_variable_digit(d)?;
            Ok(Variable::Mvxx(d))
        }
        Rule::variable_ovxx => {
            let d = pair
                .clone()
                .into_inner()
                .next()
                .ok_or(CaosError::new_parse_error(pair))?;
            let d = parse_variable_digit(d)?;
            Ok(Variable::Ovxx(d))
        }
        Rule::variable_vaxx => {
            let d = pair
                .clone()
                .into_inner()
                .next()
                .ok_or(CaosError::new_parse_error(pair))?;
            let d = parse_variable_digit(d)?;
            Ok(Variable::Vaxx(d))
        }
        Rule::variable_velx => Ok(Variable::Velx),
        Rule::variable_vely => Ok(Variable::Vely),
        Rule::variable_avar => {
            let (agent, int);
            {
                let mut it = pair.clone().into_inner();
                agent = it.next().ok_or(CaosError::new_parse_error(pair.clone()))?;
                int = it.next().ok_or(CaosError::new_parse_error(pair))?;
            }
            Ok(Variable::Avar {
                agent: super::parse_agent_arg(agent).map(Box::new)?,
                index: super::parse_int_arg(int).map(Box::new)?,
            })
        }
        Rule::variable_game => {
            let string = pair
                .clone()
                .into_inner()
                .next()
                .ok_or(CaosError::new_parse_error(pair))?;
            let string = super::parse_string_arg(string)?;
            Ok(Variable::Game {
                variable_name: Box::new(string),
            })
        }
        Rule::variable_p1 => Ok(Variable::P1),
        Rule::variable_p2 => Ok(Variable::P2),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Agent, AgentArg, IntArg, Integer, SString, SStringArg},
        parser::CaosParser,
    };
    use pest::Parser;

    use super::*;

    #[test]
    fn test_variable_mvxx() {
        for p in CaosParser::parse(Rule::variable, "MV00").expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Mvxx(0),
            );
        }
        for p in CaosParser::parse(Rule::variable, "mV32").expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Mvxx(32),
            );
        }
        for p in CaosParser::parse(Rule::variable, "mv99").expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Mvxx(99),
            );
        }
    }

    #[test]
    fn test_variable_mvxx_fail() {
        CaosParser::parse(Rule::variable, "MV3").expect_err("Not enough characters");
        CaosParser::parse(Rule::variable, "MVFF").expect_err("Incorrect characters");
    }

    #[test]
    fn test_variable_ovxx() {
        for p in CaosParser::parse(Rule::variable, "ov00").expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Ovxx(0),
            );
        }
        for p in CaosParser::parse(Rule::variable, "Ov32").expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Ovxx(32),
            );
        }
        for p in CaosParser::parse(Rule::variable, "ov99").expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Ovxx(99),
            );
        }
    }

    #[test]
    fn test_variable_ovxx_fail() {
        CaosParser::parse(Rule::variable, "OV3").expect_err("Not enough characters");
        CaosParser::parse(Rule::variable, "OVFF").expect_err("Incorrect characters");
    }

    #[test]
    fn test_variable_vaxx() {
        for p in CaosParser::parse(Rule::variable, "va00").expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Vaxx(0),
            );
        }
        for p in CaosParser::parse(Rule::variable, "va32").expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Vaxx(32),
            );
        }
        for p in CaosParser::parse(Rule::variable, "va99").expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Vaxx(99),
            );
        }
    }

    #[test]
    fn test_variable_vaxx_fail() {
        CaosParser::parse(Rule::variable, "va3").expect_err("Not enough characters");
        CaosParser::parse(Rule::variable, "vaFF").expect_err("Incorrect characters");
    }

    #[test]
    fn test_variable_game() {
        for p in CaosParser::parse(Rule::variable, r#"GAME "Hello""#).expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Game {
                    variable_name: Box::new(SStringArg::String(SString::Literal(
                        "Hello".to_owned()
                    )))
                },
            );
        }
    }

    #[test]
    fn test_variable_avar() {
        for p in CaosParser::parse(Rule::variable, r#"avar CARR 34"#).expect("Parsed") {
            assert_eq!(
                parse_variable(p).expect("Parsed variable"),
                Variable::Avar {
                    agent: Box::new(AgentArg::Agent(Agent::Carr)),
                    index: Box::new(IntArg::Primary(Integer::Literal(34)))
                },
            );
        }
    }
}
