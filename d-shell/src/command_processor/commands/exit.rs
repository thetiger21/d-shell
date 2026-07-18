use std::process::exit;

use crate::command_processor::commands::ShellCommand;

pub struct Exit;

impl ShellCommand for Exit {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }
    fn run(
        &mut self,
        _: &mut crate::state::ShellState,
        _: String,
        _: Vec<crate::token_types::Argument>,
    ) -> String {
        exit(1);
    }
}
