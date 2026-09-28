use crate::{
    command_processor::parse::{
        lexer::LexerTokens,
        parser::{
            ParserError, generate_command::generate_command, parse_arguments::parse_arguments,
        },
    },
    token_types::Token,
};

pub fn parser(tokens: Vec<LexerTokens>) -> Result<Vec<Token>, ParserError> {
    let mut command_stack: Vec<LexerTokens> = Vec::new();
    let mut output: Vec<Token> = Vec::new();
    let mut previous_token: Option<LexerTokens> = None;
    for token in tokens {
        match token {
            LexerTokens::ArrowRight => {
                if previous_token == Some(LexerTokens::Dash) {
                    command_stack.pop();
                    output.push(parse_command(&command_stack)?);
                    command_stack.clear();
                }
            }
            _ => command_stack.push(token.clone()),
        }
        previous_token = Some(token);
    }
    if !command_stack.is_empty() {
        output.push(parse_command(&command_stack)?);
        command_stack.clear();
    }
    Ok(output)
}

pub fn parse_command(tokens: &Vec<LexerTokens>) -> Result<Token, ParserError> {
    let command = match tokens.get(0) {
        Some(data) => generate_command(data.clone())?,
        None => return Err(ParserError::NothingInPositionOne),
    };
    let input = match tokens.get(1) {
        Some(LexerTokens::Identifier(input)) => Some(input.clone()),
        _ => None,
    };
    let arguments = parse_arguments(tokens.clone())?;
    Ok(Token {
        command,
        input,
        arguments,
    })
}
