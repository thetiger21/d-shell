use core::fmt;
use inline_colorization::*;

pub mod generate_command;
pub mod parse_arguments;
pub mod parser;

pub enum ParserError {
    CommandNotExist(String),
    InvalidCharacterAtPositionOne(char),
    NothingInPositionOne,
    UnexpectedRightArrow,
    UnexpectedDash,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_string: &str = match self {
            Self::CommandNotExist(command) => &format!("Command '{}' does not exist", command),
            Self::InvalidCharacterAtPositionOne(character) => {
                &format!("Character '{}' should not exist in position one", character)
            }
            Self::NothingInPositionOne => "Empty Command",
            Self::UnexpectedRightArrow => "Unexpected right arrow ('>') character in command",
            Self::UnexpectedDash => "Unexpected dash in command",
        };
        write!(
            f,
            "{color_bright_red}{style_bold}[Parsing Error]{style_reset}{color_reset}{color_red} {}",
            error_string
        )
    }
}
