use inline_colorization::*;
use std::{fs, path::Path};

use crate::{command_processor::commands::ShellCommand, state::ShellState, token_types::Argument};

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
    ) -> String {
        #[cfg(target_family = "unix")]
        {
            if input.chars().nth(0) == Some('/') {
                match fs::read_dir(Path::new(&input)) {
                    Ok(_) => {
                        state.current_directory = input.clone();
                    }
                    Err(_) => {
                        eprintln!("Directory does not exist");
                        return format!(
                            "{color_red}{style_bold}CD: Directory does not exist{style_reset}"
                        );
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
                    return "In root directory".to_string();
                } else {
                    state.current_directory.push('/');
                    return "In root directory".to_string();
                }
                return "Successfully changed directory".to_string();
            } else {
                let string_path = format!("{}/{}", state.current_directory, input);
                let path = Path::new(&string_path);
                match fs::read_dir(path) {
                    Ok(_) => {
                        state.current_directory.push('/');

                        state.current_directory.push_str(&mut input);
                    }
                    Err(_) => {
                        eprintln!("Cd: Directory doesnt exist");
                        return format!(
                            "{color_red}{style_bold}CD: Directory does not exist{style_reset}"
                        );
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
                        return format!(
                            "{color_red}{style_bold}CD: Directory does not exist{style_reset}"
                        );
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
                    return "In root directory".to_string();
                } else {
                    state.current_directory.push('\\');
                    return "In root directory".to_string();
                }
                return "Successfully changed directory".to_string();
            } else {
                let string_path = format!("{}/{}", state.current_directory, input);
                let path = Path::new(&string_path);
                match fs::read_dir(path) {
                    Ok(_) => {
                        state.current_directory.push('\\');
                        state.current_directory.push_str(&mut input);
                    }
                    Err(_) => {
                        eprintln!("Cd: Directory doesnt exist");
                        return format!(
                            "{color_red}{style_bold}CD: Directory does not exist{style_reset}"
                        );
                    }
                }
            }
        }
        "Successfully changed directory".to_string()
    }
}
