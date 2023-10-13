use crate::{
    ast::Command,
    parser::{caos_agent::parse_agent_arg, caos_int::parse_int_arg, script::parse_script_contents},
    CaosError, Rule,
};
use pest::iterators::Pair;

pub fn parse_command_econ(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command_econ {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let agent = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_agent_arg)?;

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    Ok(Command::Econ { agent, definition })
}

pub fn parse_command_enum(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command_enum {
        return Err(CaosError::new_parse_error(pair));
    }

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

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    Ok(Command::Enum {
        family,
        genus,
        species,
        definition,
    })
}

pub fn parse_command_etch(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command_etch {
        return Err(CaosError::new_parse_error(pair));
    }

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

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    Ok(Command::Etch {
        family,
        genus,
        species,
        definition,
    })
}

pub fn parse_command_esee(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command_esee {
        return Err(CaosError::new_parse_error(pair));
    }

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

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    Ok(Command::Esee {
        family,
        genus,
        species,
        definition,
    })
}

pub fn parse_command_epas(pair: Pair<Rule>) -> Result<Command, CaosError> {
    if pair.as_rule() != Rule::command_epas {
        return Err(CaosError::new_parse_error(pair));
    }

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

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;

    Ok(Command::Epas {
        family,
        genus,
        species,
        definition,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Agent, ScriptDefinition},
        parser::{caos_command::parse_command, CaosParser},
    };
    use pest::Parser;

    use super::*;

    #[test]
    fn test_command_econ_empty() {
        for p in CaosParser::parse(Rule::command, "ECON NULL NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Econ {
                    agent: Agent::Null.into(),
                    definition: ScriptDefinition::default()
                },
            );
        }
    }

    #[test]
    fn test_command_econ() {
        for p in CaosParser::parse(Rule::command, "ECON NULL BRN: DMPB NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Econ {
                    agent: Agent::Null.into(),
                    definition: ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    }
                },
            );
        }
    }

    #[test]
    fn test_command_enum_empty() {
        for p in CaosParser::parse(Rule::command, "ENUM 0 1 2 NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Enum {
                    family: 0.into(),
                    genus: 1.into(),
                    species: 2.into(),
                    definition: ScriptDefinition::default()
                },
            );
        }
    }

    #[test]
    fn test_command_etch_empty() {
        for p in CaosParser::parse(Rule::command, "ETCH 0 1 2 NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Etch {
                    family: 0.into(),
                    genus: 1.into(),
                    species: 2.into(),
                    definition: ScriptDefinition::default()
                },
            );
        }
    }

    #[test]
    fn test_command_esee_empty() {
        for p in CaosParser::parse(Rule::command, "ESEE 0 1 2 NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Esee {
                    family: 0.into(),
                    genus: 1.into(),
                    species: 2.into(),
                    definition: ScriptDefinition::default()
                },
            );
        }
    }

    #[test]
    fn test_command_epas_empty() {
        for p in CaosParser::parse(Rule::command, "EPAS 0 1 2 NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Epas {
                    family: 0.into(),
                    genus: 1.into(),
                    species: 2.into(),
                    definition: ScriptDefinition::default()
                },
            );
        }
    }

    #[test]
    fn test_command_enum() {
        for p in CaosParser::parse(Rule::command, "ENUM 0 1 2 BRN: DMPB NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Enum {
                    family: 0.into(),
                    genus: 1.into(),
                    species: 2.into(),
                    definition: ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    }
                },
            );
        }
    }

    #[test]
    fn test_command_etch() {
        for p in CaosParser::parse(Rule::command, "ETCH 0 1 2 BRN: DMPB NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Etch {
                    family: 0.into(),
                    genus: 1.into(),
                    species: 2.into(),
                    definition: ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    }
                },
            );
        }
    }

    #[test]
    fn test_command_esee() {
        for p in CaosParser::parse(Rule::command, "ESEE 0 1 2 BRN: DMPB NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Esee {
                    family: 0.into(),
                    genus: 1.into(),
                    species: 2.into(),
                    definition: ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    }
                },
            );
        }
    }

    #[test]
    fn test_command_epas() {
        for p in CaosParser::parse(Rule::command, "EPAS 0 1 2 BRN: DMPB NEXT").expect("Parsed") {
            assert_eq!(
                parse_command(p).expect("Parsed command"),
                Command::Epas {
                    family: 0.into(),
                    genus: 1.into(),
                    species: 2.into(),
                    definition: ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    }
                },
            );
        }
    }
}
