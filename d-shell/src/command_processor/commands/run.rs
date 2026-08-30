use std::fs;

use crate::{command_processor::commands::ShellCommand, luau::run_lua};

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
        if input.ends_with(".luau") {
            match run_lua(&input) {
                Ok(string) => return string,
                Err(err_msg) => return format!("{}", err_msg),
            }
        } else {
            let string_contents = match fs::read_to_string(&input) {
                Ok(data) => data,
                Err(error_msg) => {
                    return format!("Unable to open file to run script: {}", error_msg);
                }
            };
            state.parse_script(&string_contents);
        }
        "Script ran successfully".to_string()
    }
}
