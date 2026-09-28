use std::fs;

use crate::{
    command_processor::commands::{CommandError, ShellCommand},
    state::ShellState,
    token_types::Argument,
};

pub struct Mkdir;

impl ShellCommand for Mkdir {
    fn new() -> Self {
        Self
    }

    fn run(
        &mut self,
        state: &mut ShellState,
        input: String,
        _: Vec<Argument>,
    ) -> Result<String, CommandError> {
        match fs::create_dir(format!("{}/{}", state.current_directory, input)) {
            Ok(_) => (),
            Err(err_msg) => {
                return Err(CommandError::generate_error(
                    "mkdir",
                    &format!("Unable to create folder due to: {}", err_msg),
                ));
            }
        }
        Ok(String::from("Created folder successfully"))
    }

    fn validate_arguments(&self, input: Option<String>, _: Vec<Argument>) -> bool {
        match input {
            Some(_) => true,
            None => false,
        }
    }
}
