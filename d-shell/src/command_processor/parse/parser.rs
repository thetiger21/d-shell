use std::process::exit;

use inline_colorization::*;

use crate::{
    command_processor::{
        commands::{
            ShellCommand, cat::Cat, cd::CD, clear::Clear, echo::Echo, exit::Exit, help::Help,
            ls::LS, mkdir::Mkdir, rm::Remove, run::Run, touch::Touch, write::WriteCommand,
        },
        parse::lexer::LexerTokens::{self},
    },
    token_types::{Argument, ArgumentType, Token},
};

pub fn parser(input: Vec<LexerTokens>) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut previous_thingy_existed = false;
    let split_input = split(input);
    for tokens in split_input {
        if previous_thingy_existed {
            output.push(Token::Pipe);
        }
        previous_thingy_existed = true;
        match &tokens[0] {
            LexerTokens::Identifier(identifier) => {
                let processed_identifier = proccess_identifier(identifier)?;
                output.push(processed_identifier);
            }
            _ => {
                return Err(format!(
                    "{color_bright_red}{style_bold}There should be something with words there, not weird old symbols you numtpy dumpty!{style_reset}"
                ));
            }
        }
        if tokens.len() >= 2 {
            match proccess_argument(tokens[1..].to_vec()) {
                Ok(mut arguments) => {
                    output.append(&mut arguments);
                }
                Err(_) => (),
            }
        }
    }
    Ok(output)
}

pub fn split(input: Vec<LexerTokens>) -> Vec<Vec<LexerTokens>> {
    let mut output: Vec<Vec<LexerTokens>> = Vec::new();
    let mut temp_list: Vec<LexerTokens> = Vec::new();

    let mut dash_exists_before = false;
    for token in input {
        match token {
            LexerTokens::Dash => {
                dash_exists_before = true;
                temp_list.push(LexerTokens::Dash);
            }
            LexerTokens::ArrowRight => {
                if dash_exists_before {
                    // Remove the Dash we just added — this is a pipe (->)
                    temp_list.pop();
                    output.push(temp_list.clone());
                    temp_list.clear();
                    dash_exists_before = false;
                } else {
                    eprintln!(
                        "{color_bright_red}{style_bold}You cannot have '>' without '-' at the beggining so it should be '->' not '>'{style_reset}",
                    );
                    dash_exists_before = false;
                }
            }
            _ => {
                dash_exists_before = false;
                temp_list.push(token);
            }
        }
    }

    if !temp_list.is_empty() {
        output.push(temp_list);
    }

    output
}

pub fn proccess_identifier(input: &String) -> Result<Token, String> {
    match input.as_str() {
        "cd" => Ok(Token::Command(Box::new(CD::new()))),
        "help" => Ok(Token::Command(Box::new(Help::new()))),
        "ls" => Ok(Token::Command(Box::new(LS::new()))),
        "echo" => Ok(Token::Command(Box::new(Echo::new()))),
        "exit" => Ok(Token::Command(Box::new(Exit::new()))),
        "clear" => Ok(Token::Command(Box::new(Clear::new()))),
        "mkdir" => Ok(Token::Command(Box::new(Mkdir::new()))),
        "cat" => Ok(Token::Command(Box::new(Cat::new()))),
        "touch" => Ok(Token::Command(Box::new(Touch::new()))),
        "write" => Ok(Token::Command(Box::new(WriteCommand::new()))),
        "rm" => Ok(Token::Command(Box::new(Remove::new()))),
        "run" => Ok(Token::Command(Box::new(Run::new()))),
        _ => Err(format!(
            "{color_bright_red}{style_bold}Command: '{}' does not exist",
            input
        )),
    }
}

pub fn proccess_argument(input: Vec<LexerTokens>) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_content: Vec<String> = Vec::new();
    let mut expecting_flag_id = false;

    for token in input {
        match token {
            LexerTokens::Dash => {
                // Flush any pending argument before starting a new flag
                if let Some(id) = current_id.take() {
                    let content = if current_content.is_empty() {
                        ArgumentType::Nothing
                    } else if current_content.len() == 1 {
                        ArgumentType::Single(current_content.remove(0))
                    } else {
                        ArgumentType::Multiple(std::mem::take(&mut current_content))
                    };
                    output.push(Token::Argument(Argument { id, content }));
                    current_content.clear();
                }
                expecting_flag_id = true;
            }
            LexerTokens::Identifier(string) => {
                if expecting_flag_id {
                    // This identifier is the flag name (e.g., "l" from "-l")
                    current_id = Some(string);
                    expecting_flag_id = false;
                } else if current_id.is_some() {
                    // We have an active flag — this is a value for it
                    current_content.push(string);
                } else {
                    // Identifier without a flag — treat as positional input
                    output.push(Token::Input(string));
                }
            }
            LexerTokens::Comma => {
                // Comma is a separator; the next identifier still belongs
                // to the current flag's content
            }
            LexerTokens::ArrowRight => {
                return Err(format!(
                    "{color_bright_red}{style_bold}Goodness sake can't you just use the cli properly?.. you know you cannot just have '>' when you are typing arguments... if you really want to then put flipping quotation marks around it!{style_reset}"
                ));
            }
        }
    }

    // Flush the final argument if one is pending
    if let Some(id) = current_id {
        let content = if current_content.is_empty() {
            ArgumentType::Nothing
        } else if current_content.len() == 1 {
            ArgumentType::Single(current_content.remove(0))
        } else {
            ArgumentType::Multiple(std::mem::take(&mut current_content))
        };
        output.push(Token::Argument(Argument { id, content }));
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proccess_argument_single_flag() {
        let tokens = vec![LexerTokens::Dash, LexerTokens::Identifier("a".to_string())];
        let result = proccess_argument(tokens).unwrap();
        assert_eq!(result.len(), 1);
        match &result[0] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "a");
                assert_eq!(arg.content, ArgumentType::Nothing);
            }
            _ => panic!("Expected Token::Argument"),
        }
    }

    #[test]
    fn test_proccess_argument_flag_with_value() {
        let tokens = vec![
            LexerTokens::Dash,
            LexerTokens::Identifier("o".to_string()),
            LexerTokens::Identifier("output.txt".to_string()),
        ];
        let result = proccess_argument(tokens).unwrap();
        assert_eq!(result.len(), 1);
        match &result[0] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "o");
                assert_eq!(arg.content, ArgumentType::Single("output.txt".to_string()));
            }
            _ => panic!("Expected Token::Argument"),
        }
    }

    #[test]
    fn test_proccess_argument_multiple_flags() {
        let tokens = vec![
            LexerTokens::Dash,
            LexerTokens::Identifier("l".to_string()),
            LexerTokens::Dash,
            LexerTokens::Identifier("a".to_string()),
        ];
        let result = proccess_argument(tokens).unwrap();
        assert_eq!(result.len(), 2);
        match &result[0] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "l");
                assert_eq!(arg.content, ArgumentType::Nothing);
            }
            _ => panic!("Expected Token::Argument"),
        }
        match &result[1] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "a");
                assert_eq!(arg.content, ArgumentType::Nothing);
            }
            _ => panic!("Expected Token::Argument"),
        }
    }

    #[test]
    fn test_proccess_argument_flag_with_comma_values() {
        let tokens = vec![
            LexerTokens::Dash,
            LexerTokens::Identifier("I".to_string()),
            LexerTokens::Identifier("/usr/include".to_string()),
            LexerTokens::Comma,
            LexerTokens::Identifier("/usr/local/include".to_string()),
        ];
        let result = proccess_argument(tokens).unwrap();
        assert_eq!(result.len(), 1);
        match &result[0] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "I");
                assert_eq!(
                    arg.content,
                    ArgumentType::Multiple(vec![
                        "/usr/include".to_string(),
                        "/usr/local/include".to_string()
                    ])
                );
            }
            _ => panic!("Expected Token::Argument"),
        }
    }

    #[test]
    fn test_proccess_argument_empty_input() {
        let tokens = vec![];
        let result = proccess_argument(tokens).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_proccess_argument_bare_identifiers_as_input() {
        let tokens = vec![
            LexerTokens::Identifier("hello".to_string()),
            LexerTokens::Identifier("world".to_string()),
        ];
        let result = proccess_argument(tokens).unwrap();
        assert_eq!(result.len(), 2);
        match &result[0] {
            Token::Input(input) => assert_eq!(input, "hello"),
            _ => panic!("Expected Token::Input"),
        }
        match &result[1] {
            Token::Input(input) => assert_eq!(input, "world"),
            _ => panic!("Expected Token::Input"),
        }
    }

    #[test]
    fn test_parser_ls_a() {
        let tokens = vec![
            LexerTokens::Identifier("ls".to_string()),
            LexerTokens::Dash,
            LexerTokens::Identifier("a".to_string()),
        ];
        let result = parser(tokens).unwrap();
        assert_eq!(result.len(), 2, "Expected 2 tokens: command + argument");
        match &result[0] {
            Token::Command(_) => {}
            _ => panic!("Expected Token::Command as first token"),
        }
        match &result[1] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "a");
                assert_eq!(arg.content, ArgumentType::Nothing);
            }
            _ => panic!("Expected Token::Argument as second token"),
        }
    }

    #[test]
    fn test_parser_ls_l_a() {
        let tokens = vec![
            LexerTokens::Identifier("ls".to_string()),
            LexerTokens::Dash,
            LexerTokens::Identifier("l".to_string()),
            LexerTokens::Dash,
            LexerTokens::Identifier("a".to_string()),
        ];
        let result = parser(tokens).unwrap();
        assert_eq!(result.len(), 3);
        match &result[0] {
            Token::Command(_) => {}
            _ => panic!("Expected Token::Command"),
        }
        match &result[1] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "l");
                assert_eq!(arg.content, ArgumentType::Nothing);
            }
            _ => panic!("Expected Token::Argument for -l"),
        }
        match &result[2] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "a");
                assert_eq!(arg.content, ArgumentType::Nothing);
            }
            _ => panic!("Expected Token::Argument for -a"),
        }
    }

    #[test]
    fn test_parser_echo_hello() {
        let tokens = vec![
            LexerTokens::Identifier("echo".to_string()),
            LexerTokens::Identifier("hello".to_string()),
        ];
        let result = parser(tokens).unwrap();
        assert_eq!(result.len(), 2);
        match &result[0] {
            Token::Command(_) => {}
            _ => panic!("Expected Token::Command"),
        }
        match &result[1] {
            Token::Input(input) => assert_eq!(input, "hello"),
            _ => panic!("Expected Token::Input"),
        }
    }

    #[test]
    fn test_parser_ls_file_with_comma_values() {
        // The lexer folds commas into identifiers: "hello," becomes Id("hello,")
        let input = String::from("ls -file hello, hw, wh, hw");
        let lexed = crate::command_processor::parse::lexer::lexer(&input);
        let result = parser(lexed).unwrap();
        assert_eq!(result.len(), 2);
        match &result[0] {
            Token::Command(_) => {}
            _ => panic!("Expected Token::Command"),
        }
        match &result[1] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "file");
                assert_eq!(
                    arg.content,
                    ArgumentType::Multiple(vec![
                        "hello,".to_string(),
                        "hw,".to_string(),
                        "wh,".to_string(),
                        "hw".to_string()
                    ])
                );
            }
            _ => panic!("Expected Token::Argument for -file"),
        }
    }

    #[test]
    fn test_parser_pipe_between_segments() {
        let input = String::from("mkdir -> echo");
        let lexed = crate::command_processor::parse::lexer::lexer(&input);
        let result = parser(lexed).unwrap();
        assert_eq!(result.len(), 3);
        match &result[0] {
            Token::Command(_) => {}
            _ => panic!("Expected Token::Command for mkdir"),
        }
        match &result[1] {
            Token::Pipe => {}
            _ => panic!("Expected Token::Pipe at index 1"),
        }
        match &result[2] {
            Token::Command(_) => {}
            _ => panic!("Expected Token::Command for echo"),
        }
    }

    #[test]
    fn test_parser_pipe_with_arguments() {
        let input = String::from("ls -la -> echo hello");
        let lexed = crate::command_processor::parse::lexer::lexer(&input);
        let result = parser(lexed).unwrap();
        assert_eq!(result.len(), 5);
        match &result[0] {
            Token::Command(_) => {}
            _ => panic!("Expected Token::Command for ls"),
        }
        match &result[1] {
            Token::Argument(arg) => {
                assert_eq!(arg.id, "la");
                assert_eq!(arg.content, ArgumentType::Nothing);
            }
            _ => panic!("Expected Token::Argument for -la"),
        }
        match &result[2] {
            Token::Pipe => {}
            _ => panic!("Expected Token::Pipe at index 2"),
        }
        match &result[3] {
            Token::Command(_) => {}
            _ => panic!("Expected Token::Command for echo"),
        }
        match &result[4] {
            Token::Input(input) => assert_eq!(input, "hello"),
            _ => panic!("Expected Token::Input(hello)"),
        }
    }
}
