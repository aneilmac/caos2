use super::{base::parse_int_literal, parse_command};
use crate::{
    ast::{Command, EventScriptDefinition, Script, ScriptDefinition},
    CaosError, Rule,
};
use pest::iterators::Pair;

pub fn parse_script(pair: Pair<Rule>) -> Result<Script, CaosError> {
    match pair.as_rule() {
        Rule::install_script => parse_install_script(pair),
        Rule::remove_script => parse_remove_script(pair),
        Rule::event_script => parse_event_script(pair),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

fn parse_install_script(pair: Pair<Rule>) -> Result<Script, CaosError> {
    if pair.as_rule() != Rule::install_script {
        return Err(CaosError::new_parse_error(pair));
    }

    pair.clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)
        .map(Script::Install)
}

fn parse_remove_script(pair: Pair<Rule>) -> Result<Script, CaosError> {
    if pair.as_rule() != Rule::remove_script {
        return Err(CaosError::new_parse_error(pair));
    }

    pair.clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)
        .map(Script::Removal)
}

fn parse_event_script(pair: Pair<Rule>) -> Result<Script, CaosError> {
    if pair.as_rule() != Rule::event_script {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let (family, genus, species, script_number) = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_event_header)?;

    let definition = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_script_contents)?;
    Ok(Script::Event(EventScriptDefinition {
        family,
        genus,
        species,
        script_number,
        definition,
    }))
}

fn parse_event_header(pair: Pair<Rule>) -> Result<(i32, i32, i32, i32), CaosError> {
    if pair.as_rule() != Rule::event_script_header {
        return Err(CaosError::new_parse_error(pair));
    }

    let mut it = pair.clone().into_inner();

    let family = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_int_literal)?;

    let genus = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_int_literal)?;

    let species = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_int_literal)?;

    let script_number = it
        .next()
        .ok_or(CaosError::new_parse_error(pair.clone()))
        .and_then(parse_int_literal)?;

    Ok((family, genus, species, script_number))
}

pub fn parse_script_contents(pair: Pair<Rule>) -> Result<ScriptDefinition, CaosError> {
    if pair.as_rule() != Rule::script_contents {
        return Err(CaosError::new_parse_error(pair));
    }

    let commands: Result<Vec<Command>, CaosError> = pair.into_inner().map(parse_command).collect();
    commands.map(|commands| ScriptDefinition { commands })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{Command, ScriptDefinition},
        parser::CaosParser,
    };
    use pest::Parser;

    #[test]
    fn test_script_definition_empty() {
        for p in CaosParser::parse(Rule::script_contents, "").expect("Parsed") {
            assert_eq!(
                parse_script_contents(p).expect("Parsed script contents"),
                ScriptDefinition { commands: vec![] }
            );
        }
    }

    #[test]
    fn test_script_definition() {
        for p in
            CaosParser::parse(Rule::script_contents, "PRAY REFR\nGIDS ROOT INST").expect("Parsed")
        {
            assert_eq!(
                parse_script_contents(p).expect("Parsed script contents"),
                ScriptDefinition {
                    commands: vec![Command::PrayRefr, Command::GidsRoot, Command::Inst]
                }
            );
        }
    }

    #[test]
    fn test_script_install_empty_no_end_tag() {
        for p in CaosParser::parse(Rule::install_script, "ISCR").expect("Parsed") {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Install(ScriptDefinition::default()),
            );
        }
    }

    #[test]
    fn test_script_install_empty() {
        for p in CaosParser::parse(Rule::install_script, "iscr endm").expect("Parsed") {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Install(ScriptDefinition::default()),
            );
        }
    }
    #[test]
    fn test_script_install_no_end_tags() {
        for p in CaosParser::parse(Rule::install_script, "ISCR BRN: DMPB").expect("Parsed") {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Install(ScriptDefinition {
                    commands: vec![Command::BrnDmpb]
                }),
            );
        }
    }

    #[test]
    fn test_script_install() {
        for p in CaosParser::parse(Rule::install_script, "ISCR BRN: DMPB ENDM").expect("Parsed") {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Install(ScriptDefinition {
                    commands: vec![Command::BrnDmpb]
                }),
            );
        }
    }

    #[test]
    fn test_script_removal_empty_no_end_tag() {
        for p in CaosParser::parse(Rule::remove_script, "RSCR").expect("Parsed") {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Removal(ScriptDefinition::default()),
            );
        }
    }

    #[test]
    fn test_script_removal_empty() {
        for p in CaosParser::parse(Rule::remove_script, "RSCR ENDM").expect("Parsed") {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Removal(ScriptDefinition::default()),
            );
        }
    }

    #[test]
    fn test_script_removal_no_end_tag() {
        for p in CaosParser::parse(Rule::remove_script, "RSCR BRN: DMPB").expect("Parsed") {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Removal(ScriptDefinition {
                    commands: vec![Command::BrnDmpb]
                }),
            );
        }
    }

    #[test]
    fn test_script_removal() {
        for p in CaosParser::parse(Rule::remove_script, "RSCR BRN: DMPB ENDM").expect("Parsed") {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Removal(ScriptDefinition {
                    commands: vec![Command::BrnDmpb]
                }),
            );
        }
    }

    #[test]
    fn test_script_event_empty() {
        for p in CaosParser::parse(Rule::event_script, "SCRP 0 1 2 3 ENDM").expect("Parsed") {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Event(EventScriptDefinition {
                    definition: ScriptDefinition::default(),
                    family: 0,
                    genus: 1,
                    species: 2,
                    script_number: 3
                }),
            );
        }
    }

    #[test]
    fn test_script_event() {
        for p in
            CaosParser::parse(Rule::event_script, "SCRP 0 1 2 3 BRN: DMPB ENDM").expect("Parsed")
        {
            assert_eq!(
                parse_script(p).expect("Parsed command"),
                Script::Event(EventScriptDefinition {
                    definition: ScriptDefinition {
                        commands: vec![Command::BrnDmpb]
                    },
                    family: 0,
                    genus: 1,
                    species: 2,
                    script_number: 3
                }),
            );
        }
    }
}
