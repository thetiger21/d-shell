use std::fs::{self, metadata};

use crate::command_processor::commands::ShellCommand;

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
    ) -> String {
        if arguments.len() == 0 {
            let md = match metadata(format!("{}/{}", state.current_directory, input)) {
                Ok(data) => data,
                Err(err_msg) => {
                    eprintln!("Unable to delete file {} due to: {}", input, err_msg);
                    return format!("Unable to delete file {} due to: {}", input, err_msg);
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
            String::new()
        } else {
            String::new()
        }
    }
}
