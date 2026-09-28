use std::fs::{self};

use crate::command_processor::commands::{CommandError, ShellCommand};

pub struct WriteCommand;

impl ShellCommand for WriteCommand {
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
        arguments: Vec<crate::token_types::Argument>,
    ) -> Result<String, CommandError> {
        let command_name = "write";
        let files_list: Vec<String>;
        if arguments.len() == 1 {
            if arguments[0].id == "f" || arguments[0].id == "files" {
                files_list = arguments[0].content.clone()
            } else {
                return Err(CommandError::generate_error(
                    command_name,
                    "Invalid argument in write command",
                ));
            }
        } else {
            return Err(CommandError::generate_error(
                command_name,
                "Too many or too few arguments in write command",
            ));
        }
        for file in files_list {
            match fs::write(
                format!("{}/{}", state.current_directory, file),
                input.clone(),
            ) {
                Ok(_) => (),
                Err(_) => {
                    return Err(CommandError::generate_error(
                        command_name,
                        "Unable to write to file",
                    ));
                }
            }
        }
        Ok(String::new())
    }

    fn validate_arguments(
        &self,
        input: Option<String>,
        arguments: Vec<crate::token_types::Argument>,
    ) -> bool {
        match input {
            Some(_) => (),
            None => return false,
        }
        if arguments.len() == 1 {
            if arguments[0].id == "f" || arguments[0].id == "files" {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
