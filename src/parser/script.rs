use super::parse_command;
use crate::{
    ast::{Command, Script, ScriptDefinition},
    CaosError, ErrorType, Rule,
};
use pest::iterators::Pair;

pub fn parse_script(pair: Pair<Rule>) -> Result<Script, CaosError> {
    todo!()
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
    pub fn test_empty_script_contents() {
        for p in CaosParser::parse(Rule::script_contents, "").expect("Parsed") {
            assert_eq!(
                parse_script_contents(p).expect("Parsed script contents"),
                ScriptDefinition { commands: vec![] }
            );
        }
    }

    #[test]
    pub fn test_script_contents() {
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
}
