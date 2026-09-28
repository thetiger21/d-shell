use std::fs;

use d_shell_edit::run_editor;

use crate::command_processor::commands::{CommandError, ShellCommand};

pub struct Edit;

impl ShellCommand for Edit {
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
        let text = match fs::read_to_string(format!("{}/{}", state.current_directory, input)) {
            Ok(data) => data,
            Err(err_msg) => {
                return Err(CommandError::generate_error(
                    "edit",
                    &format!("Unable to read from string to due: {err_msg}"),
                ));
            }
        };
        match run_editor(
            "text",
            &text,
            &format!("{}/{}", state.current_directory, input),
        ) {
            Ok(_) => Ok(String::new()),
            Err(err_msg) => Err(CommandError::generate_error(
                "edit",
                &format!("{}", err_msg),
            )),
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
