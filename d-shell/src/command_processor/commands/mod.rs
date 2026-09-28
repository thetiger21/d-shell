use core::fmt;
use inline_colorization::*;

use crate::{state::ShellState, token_types::Argument};

pub mod cat;
pub mod cd;
pub mod clear;
pub mod echo;
pub mod edit;
pub mod exit;
pub mod help;
pub mod ls;
pub mod marella;
pub mod mkdir;
pub mod rm;
pub mod run;
pub mod touch;
pub mod write;
pub mod zip;

pub trait ShellCommand {
    fn new() -> Self
    where
        Self: Sized;
    fn run(
        &mut self,
        state: &mut ShellState,
        input: String,
        arguments: Vec<Argument>,
    ) -> Result<String, CommandError>;
    fn validate_arguments(&self, input: Option<String>, arguments: Vec<Argument>) -> bool;
}

#[derive(Debug)]
pub struct CommandError {
    command: String,
    error_msg: String,
}

impl CommandError {
    fn generate_error(command_name: &str, error_msg: &str) -> Self {
        Self {
            command: command_name.to_string(),
            error_msg: error_msg.to_string(),
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{color_bright_red}{style_bold}[{} Command Error]{color_reset}{style_reset}{color_red}{color_red} {}",
            self.command.word_to_titlecase(),
            self.error_msg
        )
    }
}
