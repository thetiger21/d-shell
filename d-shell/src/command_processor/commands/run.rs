use std::fs;

use crate::command_processor::commands::ShellCommand;

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
    ) -> String {
        let string_contents = match fs::read_to_string(&input) {
            Ok(data) => data,
            Err(error_msg) => return format!("Unable to open file to run script: {}", error_msg),
        };
        state.parse_script(&input);
        "Script ran successfully".to_string()
    }
}
