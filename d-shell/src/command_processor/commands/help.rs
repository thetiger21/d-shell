use d_shell_edit::run_editor;

use crate::{
    command_processor::commands::{CommandError, ShellCommand},
    state::ShellState,
    token_types::Argument,
};

pub struct Help;

impl ShellCommand for Help {
    fn new() -> Self {
        Self
    }
    fn run(
        &mut self,
        _: &mut ShellState,
        _: String,
        arguments: Vec<Argument>,
    ) -> Result<String, CommandError> {
        let mut help_message = String::new();
        if arguments.len() == 1 {
            if arguments[0].id == "usage_policy" {
                help_message = include_str!("help_resources/usage_policy").to_string();
                println!("{}", help_message);
            } else if arguments[0].id == "show_c" {
                help_message = include_str!("../../../../LICENSE").to_string();
                match run_editor("text", &help_message, ".temp/sfhjdssdfdsgh") {
                    Ok(_) => print!(""),
                    Err(err_msg) => {
                        return Err(CommandError::generate_error(
                            "help",
                            &format!("{}", err_msg),
                        ));
                    }
                }
            } else {
                help_message = include_str!("help_resources/help").to_string();
                println!("{}", help_message);
            }
        }
        Ok(help_message.to_string())
    }

    fn validate_arguments(&self, _: Option<String>, arguments: Vec<Argument>) -> bool {
        let mut argument_validity = true;
        for argument in arguments {
            if argument.id == "usage_policy" {
                ()
            } else {
                argument_validity = false;
            }
        }
        argument_validity
    }
}
