use crate::{command_processor::commands::ShellCommand, state::ShellState, token_types::Argument};

pub struct Echo;

impl ShellCommand for Echo {
    fn new() -> Self {
        Self
    }
    fn run(&mut self, _: &mut crate::state::ShellState, input: String, _: Vec<Argument>) -> String {
        println!("{}", input);
        input
    }
}
