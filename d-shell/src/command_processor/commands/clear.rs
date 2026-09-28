use std::error;

use crate::{
    command_processor::commands::{CommandError, ShellCommand},
    token_types::Argument,
};

pub struct Clear;

impl ShellCommand for Clear {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn run(
        &mut self,
        _: &mut crate::state::ShellState,
        _: String,
        _: Vec<Argument>,
    ) -> Result<String, CommandError> {
        match clearscreen::clear() {
            Ok(_) => Ok("Successfully cleared screen".to_string()),
            Err(err_msg) => Err(CommandError::generate_error("clear", &format!("{err_msg}"))),
        }
    }

    fn validate_arguments(&self, _: Option<String>, _: Vec<Argument>) -> bool {
        true
    }
}

pub fn clear() {}
