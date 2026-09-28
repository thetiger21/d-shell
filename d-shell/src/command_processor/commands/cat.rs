use std::fs;

use crate::{
    command_processor::commands::{CommandError, ShellCommand},
    token_types::Argument,
};

pub struct Cat;

impl ShellCommand for Cat {
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
        let command_name = "cat";

        #[cfg(target_family = "unix")]
        let contents = match fs::read_to_string(format!("{}/{}", state.current_directory, input)) {
            Ok(contents) => contents,
            Err(error_msg) => {
                return Err(CommandError::generate_error(
                    command_name,
                    &format!("{}", error_msg),
                ));
            }
        };

        #[cfg(target_os = "windows")]
        let contents = match fs::read_to_string(format!("{}/{}", state.current_directory, input)) {
            Ok(contents) => contents,
            Err(error_msg) => {
                return Err(CommandError::generate_error(
                    command_name,
                    &format!("{}", error_msg),
                ));
            }
        };
        println!("{}", contents);
        Ok(contents)
    }

    fn validate_arguments(&self, input: Option<String>, _: Vec<Argument>) -> bool {
        input.is_some_and(|path| fs::exists(&path).unwrap_or(false))
    }
}
