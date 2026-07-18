use std::fs;

use crate::command_processor::commands::ShellCommand;

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
    ) -> String {
        #[cfg(target_family = "unix")]
        let contents = match fs::read_to_string(format!("{}/{}", state.current_directory, input)) {
            Ok(contents) => contents,
            Err(error_msg) => {
                format!("Unable to run cat command due to: {}", error_msg)
            }
        };

        #[cfg(target_os = "windows")]
        let contents = match fs::read_to_string(format!("{}/{}", state.current_directory, input)) {
            Ok(contents) => contents,
            Err(error_msg) => {
                format!("Unable to run cat command due to: {}", error_msg)
            }
        };
        println!("{}", contents);
        contents
    }
}
