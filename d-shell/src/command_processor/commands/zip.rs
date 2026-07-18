use crate::command_processor::commands::ShellCommand;

pub struct Zip;

impl ShellCommand for Zip {
    fn new() -> Self {
        Self
    }

    fn run(
        &mut self,
        state: &mut crate::state::ShellState,
        input: String,
        arguments: Vec<super::Argument>,
    ) -> String {
        String::new()
    }
}
