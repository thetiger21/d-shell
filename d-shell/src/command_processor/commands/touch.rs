use std::fs::File;

use crate::command_processor::commands::ShellCommand;

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
    ) -> String {
        match File::create(format!("{}/{}", state.current_directory, input)) {
            Ok(_) => {
                return "File created successfully".to_string();
            }
            Err(error_msg) => {
                println!("File creation failed due to: {}", error_msg);
                return format!("File creation failed due to: {}", error_msg);
            }
        }
    }
}
