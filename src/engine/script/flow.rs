use crate::commands::Label;

enum CommandFlow {
    Next,
    Invoke(Label),
    Jump(Label),
    EndBlock,
}