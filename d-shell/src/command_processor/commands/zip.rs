use crate::command_processor::commands::{CommandError, ShellCommand};

pub struct Zip;

impl ShellCommand for Zip {
    fn new() -> Self {
        Self
    }

    fn run(
        &mut self,
        _: &mut crate::state::ShellState,
        _: String,
        _: Vec<super::Argument>,
    ) -> Result<String, CommandError> {
        Err(CommandError::generate_error(
            "zip",
            "Command not implemented yet - come back later ;P",
        ))
    }

    fn validate_arguments(&self, _: Option<String>, _: Vec<crate::token_types::Argument>) -> bool {
        false
    }
}
