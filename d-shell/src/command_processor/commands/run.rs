use std::fs;

use crate::command_processor::commands::{CommandError, ShellCommand};

pub struct Run;

impl ShellCommand for Run {
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
        if input.ends_with(".script") {
            let string_contents = match fs::read_to_string(&input) {
                Ok(data) => data,
                Err(error_msg) => {
                    return Err(CommandError::generate_error(
                        "run",
                        &format!("Unable to open file to run script: {}", error_msg),
                    ));
                }
            };
            state.parse_script(&string_contents);
        } else {
        }
        Ok("Script ran successfully".to_string())
    }

    fn validate_arguments(
        &self,
        input: Option<String>,
        _: Vec<crate::token_types::Argument>,
    ) -> bool {
        let file_exists = input
            .clone()
            .is_some_and(|path| fs::exists(&path).unwrap_or(false));
        if file_exists && input.unwrap_or(String::new()).ends_with(".script") {
            true
        } else {
            false
        }
    }
}
