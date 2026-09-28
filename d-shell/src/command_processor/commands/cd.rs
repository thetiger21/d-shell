use inline_colorization::*;
use std::{fs, path::Path};

use crate::{
    command_processor::commands::{CommandError, ShellCommand},
    state::ShellState,
    token_types::Argument,
};

pub struct CD;

impl ShellCommand for CD {
    fn new() -> Self {
        Self
    }

    fn run(
        &mut self,
        state: &mut ShellState,
        mut input: String,
        arguments: Vec<Argument>,
    ) -> Result<String, CommandError> {
        let command_name = "cd";
        #[cfg(target_family = "unix")]
        {
            if input.chars().nth(0) == Some('/') {
                match fs::read_dir(Path::new(&input)) {
                    Ok(_) => {
                        state.current_directory = input.clone();
                    }
                    Err(_) => {
                        return Err(CommandError::generate_error(
                            command_name,
                            "Directory does not exist",
                        ));
                    }
                }
            } else if input.chars().nth(0) == Some('.') && input.chars().nth(1) == Some('.') {
                let mut popped_value = ' ';
                let in_root = state.current_directory == "/";
                if !state.current_directory.is_empty() || in_root {
                    while popped_value != '/' {
                        popped_value = state.current_directory.pop().unwrap();
                    }
                } else if in_root {
                    self.run(state, input, arguments);
                    return Err(CommandError::generate_error(
                        command_name,
                        "In root directory - cannot go back any further",
                    ));
                } else {
                    state.current_directory.push('/');
                    return Err(CommandError::generate_error(
                        command_name,
                        "In root directory - cannot go back any further",
                    ));
                }
                return Ok("Successfully changed directory".to_string());
            } else {
                let string_path = format!("{}/{}", state.current_directory, input);
                let path = Path::new(&string_path);
                match fs::read_dir(path) {
                    Ok(_) => {
                        state.current_directory.push('/');

                        state.current_directory.push_str(&mut input);
                    }
                    Err(_) => {
                        return Err(CommandError::generate_error(
                            command_name,
                            "Directory does not exist",
                        ));
                    }
                }
            }
        }
        #[cfg(target_os = "windows")]
        {
            if input.chars().nth(0) == Some('\\') {
                match fs::read_dir(Path::new(&input)) {
                    Ok(_) => {
                        state.current_directory = input.clone();
                    }
                    Err(_) => {
                        eprintln!("Directory does not exist");
                        return Err(CommandError::generate_error(
                            command_name,
                            "Directory does not exist",
                        ));
                    }
                }
            } else if input.chars().nth(0) == Some('.') && input.chars().nth(1) == Some('.') {
                let mut popped_value = ' ';
                let in_root = state.current_directory == "\\";
                if !state.current_directory.is_empty() || in_root {
                    while popped_value != '\\' {
                        popped_value = state.current_directory.pop().unwrap();
                    }
                } else if in_root {
                    self.run(state, input, arguments);
                    return Err(CommandError::generate_error(
                        command_name,
                        "In root directory - cannot go back any further",
                    ));
                } else {
                    state.current_directory.push('\\');
                    return Err(CommandError::generate_error(
                        command_name,
                        "In root directory - cannot go back any further",
                    ));
                }
                return Ok("Successfully changed directory".to_string());
            } else {
                let string_path = format!("{}/{}", state.current_directory, input);
                let path = Path::new(&string_path);
                match fs::read_dir(path) {
                    Ok(_) => {
                        state.current_directory.push('\\');
                        state.current_directory.push_str(&mut input);
                    }
                    Err(_) => {
                        return Err(CommandError::generate_error(
                            command_name,
                            "Directory does not exist",
                        ));
                    }
                }
            }
        }
        Ok("Successfully changed directory".to_string())
    }

    fn validate_arguments(&self, input: Option<String>, _: Vec<Argument>) -> bool {
        input.is_some_and(|path| fs::exists(&path).unwrap_or(false))
    }
}
