use crate::command_processor::commands::{CommandError, ShellCommand};
use crate::token_types::Argument;
use std::fs::File;
use std::io::copy;

#[derive(Clone)]
pub struct Marella {
    url: String,
    target_file_name: String,
}

impl ShellCommand for Marella {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            url: String::new(),
            target_file_name: String::new(),
        }
    }

    fn run(
        &mut self,
        state: &mut crate::state::ShellState,
        input: String,
        arguments: Vec<crate::token_types::Argument>,
    ) -> Result<String, CommandError> {
        let command_name = "marella";
        let mut file_names: Vec<String> = Vec::new();
        if arguments.len() == 1 {
            if arguments[0].id == "o" {
                file_names = arguments[0].content.clone()
            }
        } else {
            return Err(CommandError::generate_error(
                command_name,
                "Invalid arguments to marella command.",
            ));
        }
        self.url = input;
        for file_name in file_names {
            self.target_file_name = format!("{}/{}", state.current_directory, file_name);
            match self.clone().download() {
                Ok(()) => return Ok("Successfully downloaded file".to_string()),
                Err(err_msg) => {
                    println!("Error while downloading file: {}", err_msg);
                    return Err(CommandError::generate_error(
                        command_name,
                        &format!("Error while downloading file: {}", err_msg),
                    ));
                }
            }
        }
        Err(CommandError::generate_error(command_name, ""))
    }

    fn validate_arguments(&self, input: Option<String>, arguments: Vec<Argument>) -> bool {
        match input {
            Some(_) => (),
            None => return false,
        }
        if arguments.len() == 1 {
            if arguments[0].id == "o" { true } else { false }
        } else {
            false
        }
    }
}

impl Marella {
    fn download(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut response = reqwest::blocking::get(self.url)?;
        let mut destination = File::create(self.target_file_name)?;
        copy(&mut response, &mut destination)?;
        Ok(())
    }
}
