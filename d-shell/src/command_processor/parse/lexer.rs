use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum LexerTokens {
    Identifier(String),
    Dash,
    ArrowRight,
    Comma,
}

impl fmt::Display for LexerTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dash => {
                write!(f, "-")
            }
            Self::ArrowRight => {
                write!(f, ">")
            }
            Self::Comma => {
                write!(f, ",")
            }
            Self::Identifier(identifier) => {
                write!(f, "{}", identifier)
            }
        }
    }
}

pub fn lexer(input: &String) -> Vec<LexerTokens> {
    let mut output: Vec<LexerTokens> = Vec::new();
    let mut temp_string = String::new();

    let mut quote_mode = false;

    for token in input.chars() {
        if quote_mode {
            match token {
                '"' => {
                    quote_mode = false;
                }
                _ => {
                    temp_string.push(token);
                }
            }
        } else {
            match token {
                '-' => {
                    output.push(LexerTokens::Dash);
                    if !temp_string.is_empty() {
                        output.push(LexerTokens::Identifier(temp_string.clone()));
                        temp_string.clear();
                    }
                }
                '>' => {
                    output.push(LexerTokens::ArrowRight);
                    if !temp_string.is_empty() {
                        output.push(LexerTokens::Identifier(temp_string.clone()));
                        temp_string.clear();
                    }
                }
                '"' => {
                    if !temp_string.is_empty() {
                        output.push(LexerTokens::Identifier(temp_string.clone()));
                        temp_string.clear();
                    }
                    quote_mode = true;
                }
                ',' => {
                    output.push(LexerTokens::Comma);
                    if !temp_string.is_empty() {
                        output.push(LexerTokens::Identifier(temp_string.clone()));
                        temp_string.clear();
                    }
                }
                '\n' => (),
                '\r' => (),
                ' ' => {
                    if !temp_string.is_empty() {
                        output.push(LexerTokens::Identifier(temp_string.clone()));
                        temp_string.clear();
                    }
                }
                _ => {
                    temp_string.push(token);
                }
            }
        }
    }
    if !temp_string.is_empty() {
        output.push(LexerTokens::Identifier(temp_string.clone()));
        temp_string.clear();
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_comma_emitted_as_separate_token() {
        let input = String::from("write stuff -f hello, heyo");
        let result = lexer(&input);
        assert_eq!(
            result,
            vec![
                LexerTokens::Identifier("write".to_string()),
                LexerTokens::Identifier("stuff".to_string()),
                LexerTokens::Dash,
                LexerTokens::Identifier("f".to_string()),
                // Comma is emitted before the preceding identifier
                LexerTokens::Comma,
                LexerTokens::Identifier("hello".to_string()),
                LexerTokens::Identifier("heyo".to_string()),
            ]
        );
    }

    #[test]
    fn test_lexer_write_quoted_input_with_flag_multiple_files() {
        let input = String::from("write \"I like bannana\" -f hello heyo");
        let result = lexer(&input);
        assert_eq!(
            result,
            vec![
                LexerTokens::Identifier("write".to_string()),
                LexerTokens::Identifier("I like bannana".to_string()),
                LexerTokens::Dash,
                LexerTokens::Identifier("f".to_string()),
                LexerTokens::Identifier("hello".to_string()),
                LexerTokens::Identifier("heyo".to_string()),
            ]
        );
    }

    #[test]
    fn test_lexer_write_no_quotes_multiple_files() {
        let input = String::from("write hello -f file1 file2");
        let result = lexer(&input);
        assert_eq!(
            result,
            vec![
                LexerTokens::Identifier("write".to_string()),
                LexerTokens::Identifier("hello".to_string()),
                LexerTokens::Dash,
                LexerTokens::Identifier("f".to_string()),
                LexerTokens::Identifier("file1".to_string()),
                LexerTokens::Identifier("file2".to_string()),
            ]
        );
    }
}
