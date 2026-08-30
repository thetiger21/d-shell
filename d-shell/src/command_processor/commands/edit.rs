use d_shell_edit::run_editor;

use crate::command_processor::commands::ShellCommand;

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
    ) -> String {
        run_editor(&format!("{}/{}", state.current_directory, input)).unwrap();
        String::new()
    }
}
