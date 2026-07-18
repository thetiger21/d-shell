use crossterm::{
    cursor,
    event::{
        self, Event,
        KeyCode::{self, Modifier},
        KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use inline_colorization::*;
use std::io::{self, Stdout, Write};

use crate::{
    command_processor::{
        commands::{ShellCommand, clear::Clear as ClearCommand, exit::Exit},
        parse::parse,
    },
    state::ShellState,
    wasm::WasmRuntime,
};

pub fn get_input(state: &mut ShellState, mut wasm_runtime: WasmRuntime) -> io::Result<String> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let mut command = String::new();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    let mut command_invalid = false;

    state.display(&command, command_invalid, &stdout);

    stdout.flush()?;
    loop {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            match (code, modifiers) {
                (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    let mut exit_command = Exit::new();
                    exit_command.run(state, String::new(), Vec::new());
                }
                (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                    let mut clear = ClearCommand::new();
                    clear.run(state, String::new(), Vec::new());
                }
                (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                    while !command.is_empty() && state.write_index > 0 {
                        let value = command.pop();
                        if value == Some(' ') {
                            break;
                        }
                        state.write_index -= 1;
                    }
                    state.display(&command, command_invalid, &stdout);
                    print!("\r{}", Clear(ClearType::CurrentLine));
                    stdout.flush()?;
                }
                (KeyCode::Char(character), _) => {
                    command.insert(state.write_index, character);
                    state.write_index += 1;
                    match parse(&command) {
                        Ok(_) => {
                            command_invalid = false;
                        }
                        Err(_) => {
                            command_invalid = true;
                        }
                    };
                    if command.chars().nth(0) == Some('.') {
                        command_invalid = false;
                    }
                }

                (KeyCode::Backspace, _) => {
                    if command.len() > 0 {
                        command.pop();
                        state.write_index -= 1;
                        match parse(&command) {
                            Ok(_) => {
                                command_invalid = false;
                            }
                            Err(_) => {
                                command_invalid = true;
                            }
                        };
                        print!("\r{}", Clear(ClearType::CurrentLine));
                        state.display(&command, command_invalid, &stdout);
                        stdout.flush()?;
                    }
                }
                (KeyCode::Left, _) => {
                    if state.write_index > 0 {
                        state.write_index -= 1;
                        execute!(stdout, cursor::MoveLeft(1))?;
                        stdout.flush()?;
                    }
                }
                (KeyCode::Right, _) => {
                    if state.write_index < command.len() {
                        state.write_index += 1;
                        execute!(stdout, cursor::MoveRight(1))?;
                        stdout.flush()?;
                    }
                }
                (KeyCode::Enter, _) => {
                    disable_raw_mode()?;
                    print!("\n");
                    if command.chars().nth(0) == Some('.') {
                        state.run_program(&command, &mut wasm_runtime);
                    } else {
                        match parse(&command) {
                            Ok(tokens) => {
                                state.run(tokens);
                            }
                            Err(error_msg) => {
                                eprintln!("{}", error_msg);
                            }
                        };
                    }
                    state.write_index = 0;
                    command.clear();
                    enable_raw_mode()?;
                }
                _ => (),
            }
        }
        state.display(&command, command_invalid, &stdout);
        stdout.flush()?;
    }
}

impl ShellState {
    pub fn display(&mut self, command: &String, command_invalid: bool, mut stdout: &Stdout) {
        if command_invalid {
            print!(
                "\r{color_cyan}{}{style_reset}@{color_magenta}{}{style_reset}: {color_bright_yellow}{}{style_reset}> {color_bright_red}{}{style_reset}",
                whoami::username().unwrap_or_else(|_| "<unknown>".to_string()),
                whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string()),
                self.current_directory,
                command
            );
        } else {
            print!(
                "\r{color_cyan}{}{style_reset}@{color_magenta}{}{style_reset}: {color_bright_yellow}{}{style_reset}> {color_green}{}{style_reset}",
                whoami::username().unwrap_or_else(|_| "<unknown>".to_string()),
                whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string()),
                self.current_directory,
                command
            );
        }
        let prompt_prefix = format!(
            "{}@{}: {}> ",
            whoami::username().unwrap_or_else(|_| "<unknown>".to_string()),
            whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string()),
            self.current_directory
        );
        let target_column = prompt_prefix.len() + self.write_index;
        execute!(stdout, cursor::MoveToColumn(target_column as u16)).unwrap();
    }
}
