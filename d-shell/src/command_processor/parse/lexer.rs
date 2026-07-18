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
