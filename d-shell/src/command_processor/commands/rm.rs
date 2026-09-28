use std::fs::{self, metadata};

use crate::command_processor::commands::{CommandError, ShellCommand};

pub struct Remove;

impl ShellCommand for Remove {
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
        if arguments.len() == 0 {
            let md = match metadata(format!("{}/{}", state.current_directory, input)) {
                Ok(data) => data,
                Err(err_msg) => {
                    eprintln!("Unable to delete file {} due to: {}", input, err_msg);
                    return Err(CommandError::generate_error(
                        "rm",
                        &format!("Unable to delete file {} due to: {}", input, err_msg),
                    ));
                }
            };
            if md.is_dir() {
                match fs::remove_dir_all(format!("{}/{}", state.current_directory, input)) {
                    Ok(_) => (),
                    Err(error) => eprintln!("{}", error),
                }
            } else {
                match fs::remove_file(format!("{}/{}", state.current_directory, input)) {
                    Ok(_) => (),
                    Err(error) => eprintln!("{}", error),
                }
            }
            Ok(String::new())
        } else {
            Ok(String::new())
        }
    }

    fn validate_arguments(
        &self,
        input: Option<String>,
        _: Vec<crate::token_types::Argument>,
    ) -> bool {
        input.is_some_and(|path| fs::exists(&path).unwrap_or(false))
    }
}
