use crate::{
    command_processor::parse::{lexer::lexer, parser::parser},
    state::ShellState,
};

impl ShellState {
    pub fn parse_script(&mut self, input: &str) {
        let mut command = String::new();
        for character in input.chars() {
            match character {
                '\n' => {
                    let lexed_tokens = lexer(&command);
                    match parser(lexed_tokens) {
                        Ok(parsed_value) => {
                            self.run(parsed_value);
                        }
                        Err(error_msg) => {
                            eprintln!("{}", error_msg);
                        }
                    };
                    command.clear();
                }
                _ => {
                    command.push(character);
                }
            }
        }
    }
}
