use crate::{command_processor::commands::ShellCommand, state::ShellState, token_types::Argument};

const HELP: &str = include_str!("help");

pub struct Help;

impl ShellCommand for Help {
    fn new() -> Self {
        Self
    }
    fn run(&mut self, _: &mut ShellState, _: String, _: Vec<Argument>) -> String {
        println!("{}", HELP);
        HELP.to_string()
    }
}
