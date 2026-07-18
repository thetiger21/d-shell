use std::{fs::File, io::Write};

use crate::{command_processor::commands::ShellCommand, token_types::ArgumentType};

pub struct WriteCommand;

impl ShellCommand for WriteCommand {
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
        arguments: Vec<crate::token_types::Argument>,
    ) -> String {
        if arguments.len() >= 1 {
            let argument = &arguments[0];
            let mut files_to_write_to: Vec<String> = Vec::new();
            if argument.id == "f" || argument.id == "file" {
                match &argument.content {
                    ArgumentType::Multiple(strings) => {
                        files_to_write_to.append(&mut strings.clone());
                    }
                    ArgumentType::Single(string) => {
                        files_to_write_to.push(string.clone());
                    }
                    ArgumentType::Nothing => {
                        return "No Argument; Argument needed in order to write to file"
                            .to_string();
                    }
                }
                for file_name in files_to_write_to {
                    let mut file =
                        match File::create(format!("{}/{}", state.current_directory, file_name)) {
                            Ok(data) => data,
                            Err(error_msg) => {
                                eprintln!("Unable to create/open file due to: {}", error_msg);
                                return format!("Unable to create/open file due to: {}", error_msg);
                            }
                        };
                    match file.write_all(input.as_bytes()) {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!("Unable to write to file: {}", file_name);
                        }
                    }
                }
            }
            return "No Argument; Argument needed in order to write to file".to_string();
        } else {
            eprintln!("No Argument; Argument needed in order to write to files");
            return "No Argument; Argument needed in order to write to file".to_string();
        }
    }
}
