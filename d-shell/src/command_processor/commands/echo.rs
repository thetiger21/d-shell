use crate::{
    command_processor::commands::{CommandError, ShellCommand},
    token_types::Argument,
};

pub struct Echo;

impl ShellCommand for Echo {
    fn new() -> Self {
        Self
    }
    fn run(
        &mut self,
        _: &mut crate::state::ShellState,
        input: String,
        _: Vec<Argument>,
    ) -> Result<String, CommandError> {
        println!("{}", input);
        Ok(input)
    }

    fn validate_arguments(&self, input: Option<String>, _: Vec<Argument>) -> bool {
        match input {
            Some(_) => true,
            None => false,
        }
    }
}
