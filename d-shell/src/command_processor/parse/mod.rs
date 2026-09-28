use crate::{
    command_processor::parse::{
        lexer::lexer,
        parser::{ParserError, parser::parser},
    },
    token_types::Token,
};

pub mod lexer;
pub mod parser;

pub fn parse(input: &String) -> Result<Vec<Token>, ParserError> {
    let lexed = lexer(input);
    let parsed = parser(lexed)?;
    Ok(parsed)
}
