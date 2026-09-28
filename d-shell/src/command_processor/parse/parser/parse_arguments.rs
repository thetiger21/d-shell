use crate::{
    command_processor::parse::{lexer::LexerTokens, parser::ParserError},
    token_types::Argument,
};

pub fn parse_arguments(input: Vec<LexerTokens>) -> Result<Vec<Argument>, ParserError> {
    let mut output: Vec<Argument> = Vec::new();
    let mut arguments_stack: Vec<String> = Vec::new();
    let mut absorb_into_argument = false;
    for token in input {
        match token {
            LexerTokens::Dash => {
                if absorb_into_argument && arguments_stack.len() >= 2 {
                    output.push(Argument {
                        id: arguments_stack[0].clone(),
                        content: arguments_stack[1..].to_vec(),
                    });
                    arguments_stack.clear();
                } else if absorb_into_argument && arguments_stack.len() == 1 {
                    output.push(Argument {
                        id: arguments_stack[0].clone(),
                        content: Vec::new(),
                    });
                    arguments_stack.clear();
                } else if absorb_into_argument {
                    return Err(ParserError::UnexpectedDash);
                } else {
                    absorb_into_argument = true;
                }
            }
            LexerTokens::Identifier(argument) => {
                if absorb_into_argument {
                    arguments_stack.push(argument);
                }
            }
            LexerTokens::ArrowRight => {
                return Err(ParserError::UnexpectedRightArrow);
            }
            LexerTokens::Comma => (),
        }
    }
    if absorb_into_argument && arguments_stack.len() >= 2 {
        output.push(Argument {
            id: arguments_stack[0].clone(),
            content: arguments_stack[1..].to_vec(),
        });
        arguments_stack.clear();
    } else if absorb_into_argument && arguments_stack.len() == 1 {
        output.push(Argument {
            id: arguments_stack[0].clone(),
            content: Vec::new(),
        });
        arguments_stack.clear();
    } else if absorb_into_argument {
        return Err(ParserError::UnexpectedDash);
    }
    Ok(output)
}
