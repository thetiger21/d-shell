use std::fs;

use crate::{command_processor::commands::ShellCommand, state::ShellState, token_types::Argument};

pub struct Mkdir;

impl ShellCommand for Mkdir {
    fn new() -> Self {
        Self
    }

    fn run(&mut self, state: &mut ShellState, input: String, arguments: Vec<Argument>) -> String {
        match fs::create_dir(format!("{}/{}", state.current_directory, input)) {
            Ok(_) => (),
            Err(err_msg) => {
                return "[ERROR] Unable to create folder due to: {}".to_string();
            }
        }
        String::from("Created folder successfully")
    }
}
