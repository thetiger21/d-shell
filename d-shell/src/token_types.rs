use crate::command_processor::commands::ShellCommand;

pub enum Token {
    Command(Box<dyn ShellCommand>),
    Argument(Argument),
    Input(String),
    Pipe,
}

pub struct Argument {
    pub id: String,
    pub content: ArgumentType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArgumentType {
    Single(String),
    Multiple(Vec<String>),
    Nothing,
}
