use std::{collections::HashMap, env};

#[derive(Clone)]
pub struct ShellState {
    pub current_directory: String,
    programs_directory: String,
    history: Vec<String>,
    pub write_index: usize,
    pub output: String,
    pub variables: HashMap<String, String>,
}

impl ShellState {
    pub fn new() -> Self {
        let current_directory = match env::current_dir() {
            Ok(data) => data.display().to_string(),
            Err(err) => {
                eprintln!("Unable to get current path: {}", err);
                String::new()
            }
        };
        #[cfg(target_family = "unix")]
        let programs_directory = format!("{}/.programs", current_directory,);
        #[cfg(target_os = "windows")]
        let programs_directory = format!("{}\\.programs", current_directory,);
        Self {
            current_directory,
            programs_directory,
            write_index: 0,
            history: Vec::new(),
            output: String::new(),
            variables: HashMap::new(),
        }
    }

    pub fn get_history(&mut self, index: usize) -> Option<String> {
        if self.history.len() <= index - 1 {
            return Some(self.history[index].clone());
        } else {
            return None;
        }
    }

    pub fn get_programs_directory(self) -> String {
        self.programs_directory
    }
}
