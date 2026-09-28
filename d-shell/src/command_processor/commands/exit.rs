use std::process::exit;

use crate::command_processor::commands::{CommandError, ShellCommand};

pub struct Exit;

impl ShellCommand for Exit {
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
        _: Vec<crate::token_types::Argument>,
    ) -> Result<String, CommandError> {
        exit(1);
    }

    fn validate_arguments(&self, _: Option<String>, _: Vec<crate::token_types::Argument>) -> bool {
        true
    }
}
