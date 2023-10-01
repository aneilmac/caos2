use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/base.pest"]
#[grammar = "grammar/caos_agent.pest"]
#[grammar = "grammar/caos_command.pest"]
#[grammar = "grammar/caos_float.pest"]
#[grammar = "grammar/caos_int.pest"]
#[grammar = "grammar/caos_program.pest"]
#[grammar = "grammar/caos_string.pest"]
#[grammar = "grammar/caos_variable.pest"]
#[grammar = "grammar/condition.pest"]
#[grammar = "grammar/script.pest"]
struct CaosParser;
