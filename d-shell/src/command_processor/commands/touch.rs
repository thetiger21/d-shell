use std::fs::File;

use crate::command_processor::commands::{CommandError, ShellCommand};

pub struct Touch;

impl ShellCommand for Touch {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn run(
        &mut self,
        state: &mut crate::state::ShellState,
        input: String,
        _: Vec<crate::token_types::Argument>,
    ) -> Result<String, CommandError> {
        match File::create(format!("{}/{}", state.current_directory, input)) {
            Ok(_) => {
                return Ok("File created successfully".to_string());
            }
            Err(error_msg) => {
                return Err(CommandError::generate_error(
                    "touch",
                    &format!("File creation failed due to: {}", error_msg),
                ));
            }
        }
    }

    fn validate_arguments(
        &self,
        input: Option<String>,
        _: Vec<crate::token_types::Argument>,
    ) -> bool {
        match input {
            Some(_) => true,
            None => false,
        }
    }
}
