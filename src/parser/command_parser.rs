#[cfg(test)]
mod tests;

mod command_parser_trait;
mod command_thunk;

use crate::{ast::Command, parser::parse_condition, CaosError, Rule};
pub(crate) use command_parser_trait::CommandParser;
pub(crate) use command_thunk::CommandThunk;
use pest::iterators::{Pair, Pairs};

use super::{base::parse_label, parse_expression};

pub fn parse_commands<'i>(pairs: &mut Pairs<'i, Rule>) -> Result<Vec<Command>, CaosError> {
    let mut commands = Vec::<Command>::new();

    while let Some(pair) = pairs.next() {
        let mut thunk: CommandThunk = find_command_match(pair, pairs)?;

        loop {
            if thunk.is_ready() {
                let c = thunk.complete()?;
                commands.push(c);
                break;
            } else {
                match thunk {
                    CommandThunk::Completed(..) => unreachable!(),
                    CommandThunk::Partial(ref mut p) => {
                        let arg = parse_expression(pairs)?;
                        p.arg_parts.push(arg);
                    }
                }
            }
        }
    }
    Ok(commands)
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
        _ => None,
    }
    .or_else(|| Command::parse_thunk(pair.clone()))
    .ok_or_else(|| CaosError::new_parse_error(pair))
}
