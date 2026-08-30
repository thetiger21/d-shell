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
                return "Successfully wrote to file".to_string();
            }
            return "No Argument; Argument needed in order to write to file".to_string();
        } else {
            eprintln!("No Argument; Argument needed in order to write to files");
            return "No Argument; Argument needed in order to write to file".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::state::ShellState;
    use crate::token_types::{Argument, ArgumentType};

    fn setup_state() -> (ShellState, tempfile::TempDir) {
        let dir = tempfile::tempdir().expect("Failed to create temp dir");
        let path = dir.path().to_str().unwrap().to_string();
        let mut state = ShellState::new();
        state.current_directory = path;
        (state, dir)
    }

    #[test]
    fn test_write_command_single_file() {
        let (mut state, dir) = setup_state();
        let mut cmd = WriteCommand::new();
        let result = cmd.run(
            &mut state,
            "Hello, World!".to_string(),
            vec![Argument {
                id: "f".to_string(),
                content: ArgumentType::Single("test_output.txt".to_string()),
            }],
        );
        assert_eq!(result, "Successfully wrote to file");
        let file_path = dir.path().join("test_output.txt");
        let contents = fs::read_to_string(&file_path)
            .unwrap_or_else(|_| panic!("Failed to read {:?}", file_path));
        assert_eq!(contents, "Hello, World!");
    }

    #[test]
    fn test_write_command_with_quoted_input() {
        let (mut state, dir) = setup_state();
        let mut cmd = WriteCommand::new();
        let result = cmd.run(
            &mut state,
            "I like bannana".to_string(),
            vec![Argument {
                id: "f".to_string(),
                content: ArgumentType::Single("hello".to_string()),
            }],
        );
        assert_eq!(result, "Successfully wrote to file");
        let file_path = dir.path().join("hello");
        let contents = fs::read_to_string(&file_path)
            .unwrap_or_else(|_| panic!("Failed to read {:?}", file_path));
        assert_eq!(contents, "I like bannana");
    }

    #[test]
    fn test_write_command_flag_file() {
        let (mut state, dir) = setup_state();
        let mut cmd = WriteCommand::new();
        let result = cmd.run(
            &mut state,
            "some content".to_string(),
            vec![Argument {
                id: "file".to_string(),
                content: ArgumentType::Single("test.txt".to_string()),
            }],
        );
        assert_eq!(result, "Successfully wrote to file");
        let file_path = dir.path().join("test.txt");
        let contents = fs::read_to_string(&file_path)
            .unwrap_or_else(|_| panic!("Failed to read {:?}", file_path));
        assert_eq!(contents, "some content");
    }

    #[test]
    fn test_write_command_no_argument() {
        let (mut state, _dir) = setup_state();
        let mut cmd = WriteCommand::new();
        let result = cmd.run(
            &mut state,
            "content".to_string(),
            vec![Argument {
                id: "f".to_string(),
                content: ArgumentType::Nothing,
            }],
        );
        assert_eq!(
            result,
            "No Argument; Argument needed in order to write to file"
        );
    }

    #[test]
    fn test_write_command_missing_flag_returns_no_argument() {
        let (mut state, _dir) = setup_state();
        let mut cmd = WriteCommand::new();
        let result = cmd.run(
            &mut state,
            "content".to_string(),
            vec![Argument {
                id: "x".to_string(),
                content: ArgumentType::Single("file.txt".to_string()),
            }],
        );
        assert_eq!(
            result,
            "No Argument; Argument needed in order to write to file"
        );
    }

    #[test]
    fn test_write_command_empty_arguments_returns_no_argument() {
        let (mut state, _dir) = setup_state();
        let mut cmd = WriteCommand::new();
        let result = cmd.run(&mut state, "content".to_string(), vec![]);
        assert_eq!(
            result,
            "No Argument; Argument needed in order to write to file"
        );
    }

    #[test]
    fn test_write_command_multiple_files_all_written() {
        // Both files should be written since the for loop iterates all files
        // before returning success.
        let (mut state, dir) = setup_state();
        let mut cmd = WriteCommand::new();
        let result = cmd.run(
            &mut state,
            "I like bannana".to_string(),
            vec![Argument {
                id: "f".to_string(),
                content: ArgumentType::Multiple(vec!["hello".to_string(), "heyo".to_string()]),
            }],
        );
        assert_eq!(result, "Successfully wrote to file");

        let file1_path = dir.path().join("hello");
        assert!(
            file1_path.exists(),
            "Expected first file 'hello' to be created"
        );
        let contents1 = fs::read_to_string(&file1_path)
            .unwrap_or_else(|_| panic!("Failed to read {:?}", file1_path));
        assert_eq!(contents1, "I like bannana");

        let file2_path = dir.path().join("heyo");
        assert!(
            file2_path.exists(),
            "Expected second file 'heyo' to be created"
        );
        let contents2 = fs::read_to_string(&file2_path)
            .unwrap_or_else(|_| panic!("Failed to read {:?}", file2_path));
        assert_eq!(contents2, "I like bannana");
    }
}
