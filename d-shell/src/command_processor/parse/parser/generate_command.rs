use crate::command_processor::{
    commands::{
        ShellCommand, cat::Cat, cd::CD, clear::Clear, echo::Echo, edit::Edit, exit::Exit,
        help::Help, ls::LS, marella::Marella, mkdir::Mkdir, rm::Remove, run::Run, touch::Touch,
        write::WriteCommand, zip::Zip,
    },
    parse::{lexer::LexerTokens, parser::ParserError},
};

pub fn generate_command(token: LexerTokens) -> Result<Box<dyn ShellCommand>, ParserError> {
    match token {
        LexerTokens::Identifier(command) => match command.as_str() {
            "cat" => Ok(Box::new(Cat::new())),
            "cd" => Ok(Box::new(CD::new())),
            "clear" => Ok(Box::new(Clear::new())),
            "echo" => Ok(Box::new(Echo::new())),
            "edit" => Ok(Box::new(Edit::new())),
            "exit" => Ok(Box::new(Exit::new())),
            "help" => Ok(Box::new(Help::new())),
            "ls" => Ok(Box::new(LS::new())),
            "marella" => Ok(Box::new(Marella::new())),
            "mkdir" => Ok(Box::new(Mkdir::new())),
            "rm" => Ok(Box::new(Remove::new())),
            "run" => Ok(Box::new(Run::new())),
            "touch" => Ok(Box::new(Touch::new())),
            "write" => Ok(Box::new(WriteCommand::new())),
            "zip" => Ok(Box::new(Zip::new())),
            _ => Err(ParserError::CommandNotExist(command)),
        },
        LexerTokens::ArrowRight => Err(ParserError::InvalidCharacterAtPositionOne('>')),
        LexerTokens::Comma => Err(ParserError::InvalidCharacterAtPositionOne(',')),
        LexerTokens::Dash => Err(ParserError::InvalidCharacterAtPositionOne('-')),
    }
}
