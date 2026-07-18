use crate::{command_processor::commands::ShellCommand, state::ShellState, token_types::Argument};

pub struct Clear;

impl ShellCommand for Clear {
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
        arguments: Vec<Argument>,
    ) -> String {
        match clearscreen::clear() {
            Ok(_) => "Successfully cleared screen".to_string(),
            Err(err_msg) => format!("Unable to clear screen due to: {}", err_msg),
        }
    }
}

pub fn clear() {}
