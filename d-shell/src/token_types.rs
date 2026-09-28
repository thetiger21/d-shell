use crate::command_processor::commands::ShellCommand;

pub struct Token {
    pub command: Box<dyn ShellCommand>,
    pub input: Option<String>,
    pub arguments: Vec<Argument>,
}

pub struct Argument {
    pub id: String,
    pub content: Vec<String>,
}
