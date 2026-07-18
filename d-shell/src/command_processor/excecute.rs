use crate::{
    command_processor::commands::ShellCommand,
    state::ShellState,
    token_types::{Argument, Token},
};

impl ShellState {
    pub fn run(&mut self, command: Vec<Token>) {
        let mut command_to_run: Option<Box<dyn ShellCommand>> = None;
        let mut input: String = String::new();
        let mut arguments: Vec<Argument> = Vec::new();

        for token in command {
            match token {
                Token::Command(cmd) => command_to_run = Some(cmd),
                Token::Input(input_recieved) => input = input_recieved,
                Token::Argument(argument) => arguments.push(argument),
                Token::Pipe => {
                    if let Some(cmd) = command_to_run.take() {
                        self.execute(
                            cmd,
                            std::mem::take(&mut input),
                            std::mem::take(&mut arguments),
                        );
                    }
                    arguments.clear()
                }
            }
        }

        // Execute the final command if there was no Pipe
        if let Some(cmd) = command_to_run {
            self.execute(cmd, input, arguments);
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
        self.output = output;
    }
}
