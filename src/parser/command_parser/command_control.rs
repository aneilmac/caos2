use pest::iterators::Pair;

use crate::{
    ast::{AgentArg, ClassifierEnum, Command, DoIf, IntArg, Label, ScriptDefinition},
    CaosError, Rule,
};

pub enum Control {
    Subr {
        label: Label,
        definition: ScriptDefinition,
    },
    Reps {
        count: Box<IntArg>,
        definition: ScriptDefinition,
    },
    Loop {
        definition: ScriptDefinition,
    },
    DoIf(DoIf),
    Econ {
        agent: Box<AgentArg>,
        definition: ScriptDefinition,
    },
    Enum(ClassifierEnum),
    Etch(ClassifierEnum),
    Esee(ClassifierEnum),
    Epas(ClassifierEnum),
}

impl Control {
    pub fn push(&mut self, command: Command) {
        match self {
            Control::Subr { definition, .. } => definition.push(command),
            Control::Reps { definition, .. } => definition.push(command),
            Control::Loop { definition } => definition.push(command),
            Control::Econ { definition, .. } => definition.push(command),
            Control::Enum(ClassifierEnum { definition, .. }) => definition.push(command),
            Control::Etch(ClassifierEnum { definition, .. }) => definition.push(command),
            Control::Esee(ClassifierEnum { definition, .. }) => definition.push(command),
            Control::Epas(ClassifierEnum { definition, .. }) => definition.push(command),
            Control::DoIf(do_if) => do_if.push(command),
        }
    }
}
