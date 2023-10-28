#[cfg(test)]
mod tests;

mod command_control;
mod command_parser_trait;
mod command_stack;
mod command_thunk;

use crate::{
    ast::{AgentArg, ClassifierEnum, Command, DoIf, IntArg, ScriptDefinition},
    parser::parse_condition,
    CaosError, Rule,
};
use command_control::Control;
pub(crate) use command_parser_trait::CommandParser;
pub(crate) use command_thunk::CommandThunk;
use pest::iterators::{Pair, Pairs};

use self::command_stack::CommandStack;

use super::{base::parse_label, parse_expression};

pub fn parse_commands<'i>(pairs: &mut Pairs<'i, Rule>) -> Result<Vec<Command>, CaosError> {
    let mut command_stack = CommandStack::new();

    while let Some(pair) = pairs.next() {
        let mut thunk: CommandThunk = find_command_match(pair, pairs)?;

        loop {
            if thunk.needs_expression() {
                match thunk {
                    CommandThunk::Partial(ref mut p) => {
                        let arg = parse_expression(pairs)?;
                        p.arg_parts.push(arg);
                    }
                    _ => unreachable!(),
                }
            } else {
                command_stack.push(thunk)?;
                break;
            }
        }
    }
    command_stack.to_commands()
}

fn find_command_match<'i>(
    pair: Pair<'i, Rule>,
    remainder: &mut Pairs<'i, Rule>,
) -> Result<CommandThunk<'i>, CaosError> {
    let rule = pair.as_rule();
    match rule {
        Rule::command_dbg_asrt => {
            let condition = parse_condition(remainder)?;
            Some(CommandThunk::Completed(
                pair.clone(),
                Command::DbgAsrt { condition },
            ))
        }
        Rule::command_gsub => {
            let label_p = remainder
                .next()
                .ok_or_else(|| CaosError::new_end_of_stream())?;
            let destination = parse_label(label_p)?;
            Some(CommandThunk::Completed(
                pair.clone(),
                Command::Gsub { destination },
            ))
        }
        Rule::command_goto => {
            let label_p = remainder
                .next()
                .ok_or_else(|| CaosError::new_end_of_stream())?;
            let destination = parse_label(label_p)?;
            Some(CommandThunk::Completed(
                pair.clone(),
                Command::Goto { destination },
            ))
        }
        Rule::command_doif => {
            let condition = parse_condition(remainder)?;
            Some(CommandThunk::Start(Control::DoIf(DoIf::empty(condition))))
        }
        Rule::command_elif => {
            let condition = parse_condition(remainder)?;
            Some(CommandThunk::StartElif(pair.clone(), condition))
        }
        Rule::command_else => Some(CommandThunk::StartElse(pair.clone())),
        Rule::command_subr => {
            let label_p = remainder
                .next()
                .ok_or_else(|| CaosError::new_end_of_stream())?;
            let label = parse_label(label_p)?;
            Some(CommandThunk::Start(Control::Subr {
                label,
                definition: ScriptDefinition::default(),
            }))
        }
        Rule::command_reps => {
            let count: IntArg = parse_expression(remainder)?.try_into()?;
            Some(CommandThunk::Start(Control::Reps {
                count: Box::new(count),
                definition: ScriptDefinition::default(),
            }))
        }
        Rule::command_loop => Some(CommandThunk::Start(Control::Loop {
            definition: ScriptDefinition::default(),
        })),
        Rule::command_econ => {
            let agent: AgentArg = parse_expression(remainder)?.try_into()?;
            Some(CommandThunk::Start(Control::Econ {
                agent: Box::new(agent),
                definition: ScriptDefinition::default(),
            }))
        }
        Rule::command_enum => {
            let classifer = parse_enum_classifier(remainder)?;
            Some(CommandThunk::Start(Control::Enum(classifer)))
        }
        Rule::command_etch => {
            let classifer = parse_enum_classifier(remainder)?;
            Some(CommandThunk::Start(Control::Etch(classifer)))
        }
        Rule::command_esee => {
            let classifer = parse_enum_classifier(remainder)?;
            Some(CommandThunk::Start(Control::Esee(classifer)))
        }
        Rule::command_epas => {
            let classifer = parse_enum_classifier(remainder)?;
            Some(CommandThunk::Start(Control::Epas(classifer)))
        }
        Rule::command_untl => {
            let condition = parse_condition(remainder)?;
            Some(CommandThunk::EndLoop(pair.clone(), condition))
        }
        Rule::command_endi
        | Rule::command_retn
        | Rule::command_ever
        | Rule::command_next
        | Rule::command_repe => Some(CommandThunk::End(pair.clone())),
        _ => None,
    }
    .or_else(|| Command::parse_thunk(pair.clone()))
    .ok_or_else(|| CaosError::new_parse_error(pair))
}

fn parse_enum_classifier(remainder: &mut Pairs<Rule>) -> Result<ClassifierEnum, CaosError> {
    let family: Box<IntArg> = Box::new(parse_expression(remainder)?.try_into()?);
    let genus: Box<IntArg> = Box::new(parse_expression(remainder)?.try_into()?);
    let species: Box<IntArg> = Box::new(parse_expression(remainder)?.try_into()?);
    Ok(ClassifierEnum {
        family,
        genus,
        species,
        definition: ScriptDefinition::default(),
    })
}
