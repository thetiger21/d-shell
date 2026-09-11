use crate::command_processor::commands::ShellCommand;
use crate::token_types::{Argument, ArgumentType};
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
    ) -> String {
        let mut file_name = &String::new();
        match &arguments[0].content {
            ArgumentType::Single(argument) => {
                if arguments[0].id == "o" {
                    file_name = argument;
                }
            }
            ArgumentType::Multiple(argument) => {}
            ArgumentType::Nothing => {}
        }
        self.url = input;
        self.target_file_name = format!("{}/{}", state.current_directory, file_name);
        match self.clone().download() {
            Ok(()) => return format!("Successfully downloaded file"),
            Err(err_msg) => {
                println!("Error while downloading file: {}", err_msg);
                return format!("Error while dwnloading file: {}", err_msg);
            }
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
