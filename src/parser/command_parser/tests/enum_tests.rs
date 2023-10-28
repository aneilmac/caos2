use super::*;
use crate::ast::{Agent, ClassifierEnum, ScriptDefinition};
use pest::Parser;

#[test]
fn test_command_econ_error_no_next() {
    let mut p = CaosParser::parse(Rule::command, "ECON NULL").expect("Successful parse");
    parse_commands(&mut p).expect_err("No NEXT");
}

#[test]
fn test_command_enum_error_no_next() {
    let mut p = CaosParser::parse(Rule::command, "ENUM 0 1 2").expect("Successful parse");
    parse_commands(&mut p).expect_err("No NEXT");
}

#[test]
fn test_command_etch_error_no_next() {
    let mut p = CaosParser::parse(Rule::command, "ETCH 0 1 2").expect("Successful parse");
    parse_commands(&mut p).expect_err("No NEXT");
}

#[test]
fn test_command_esee_error_no_next() {
    let mut p = CaosParser::parse(Rule::command, "ESEE 0 1 2").expect("Successful parse");
    parse_commands(&mut p).expect_err("No NEXT");
}

#[test]
fn test_command_epas_error_no_next() {
    let mut p = CaosParser::parse(Rule::command, "EPAS 0 1 2").expect("Successful parse");
    parse_commands(&mut p).expect_err("No NEXT");
}

#[test]
fn test_command_econ_empty() {
    assert_eq!(
        parse_cmnd("ECON NULL NEXT"),
        Command::Econ {
            agent: Box::new(Agent::Null.into()),
            definition: ScriptDefinition::default()
        },
    );
}

#[test]
fn test_command_econ() {
    assert_eq!(
        parse_cmnd("ECON NULL BRN: DMPB NEXT"),
        Command::Econ {
            agent: Box::new(Agent::Null.into()),
            definition: ScriptDefinition {
                commands: vec![Command::BrnDmpb]
            }
        },
    );
}

#[test]
fn test_command_enum_empty() {
    assert_eq!(
        parse_cmnd("ENUM 0 1 2 NEXT"),
        Command::Enum(ClassifierEnum {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            definition: ScriptDefinition::default()
        },)
    );
}

#[test]
fn test_command_etch_empty() {
    assert_eq!(
        parse_cmnd("ETCH 0 1 2 NEXT"),
        Command::Etch(ClassifierEnum {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            definition: ScriptDefinition::default()
        },)
    );
}

#[test]
fn test_command_esee_empty() {
    assert_eq!(
        parse_cmnd("ESEE 0 1 2 NEXT"),
        Command::Esee(ClassifierEnum {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            definition: ScriptDefinition::default()
        },)
    );
}

#[test]
fn test_command_epas_empty() {
    assert_eq!(
        parse_cmnd("EPAS 0 1 2 NEXT"),
        Command::Epas(ClassifierEnum {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            definition: ScriptDefinition::default()
        },)
    );
}

#[test]
fn test_command_enum() {
    assert_eq!(
        parse_cmnd("ENUM 0 1 2 BRN: DMPB NEXT"),
        Command::Enum(ClassifierEnum {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            definition: ScriptDefinition {
                commands: vec![Command::BrnDmpb]
            }
        },)
    );
}

#[test]
fn test_command_etch() {
    assert_eq!(
        parse_cmnd("ETCH 0 1 2 BRN: DMPB NEXT"),
        Command::Etch(ClassifierEnum {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            definition: ScriptDefinition {
                commands: vec![Command::BrnDmpb]
            }
        },)
    );
}

#[test]
fn test_command_esee() {
    assert_eq!(
        parse_cmnd("ESEE 0 1 2 BRN: DMPB NEXT"),
        Command::Esee(ClassifierEnum {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            definition: ScriptDefinition {
                commands: vec![Command::BrnDmpb]
            }
        },)
    );
}

#[test]
fn test_command_epas() {
    assert_eq!(
        parse_cmnd("EPAS 0 1 2 BRN: DMPB NEXT"),
        Command::Epas(ClassifierEnum {
            family: Box::new(0.into()),
            genus: Box::new(1.into()),
            species: Box::new(2.into()),
            definition: ScriptDefinition {
                commands: vec![Command::BrnDmpb]
            }
        },)
    );
}
