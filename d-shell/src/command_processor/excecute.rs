use crate::{
    command_processor::commands::ShellCommand,
    state::ShellState,
    token_types::{Argument, Token},
};

impl ShellState {
    pub fn run(&mut self, commands: Vec<Token>) {
        for command in commands {
            let input = match command.input {
                Some(input) => input,
                None => String::new(),
            };
            self.execute(command.command, input, command.arguments);
        }
    }

    pub fn execute(
        &mut self,
        mut command: Box<dyn ShellCommand>,
        input_arg: String,
        arguments: Vec<Argument>,
    ) {
        let input: String = if !input_arg.is_empty() {
            input_arg
        } else {
            self.output.clone()
        };
        let output = command.run(self, input, arguments);
        match output {
            Ok(command_results) => self.output = command_results,
            Err(err_msg) => {
                eprintln!("{}", err_msg)
            }
        }
    }
}
