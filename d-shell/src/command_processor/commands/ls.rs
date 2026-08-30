use std::{
    process::exit,
    time::{Duration, SystemTime},
};

use comfy_table::{
    Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL_CONDENSED,
};

use crate::{command_processor::commands::ShellCommand, state::ShellState, token_types::Argument};

pub struct LS {
    elements: Vec<File>,
}

pub struct File {
    name: String,
    size: u64,
    data_type: Type,
    last_modified: SystemTime,
}

#[derive(PartialEq)]
pub enum Type {
    File,
    Folder,
}

impl ShellCommand for LS {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            elements: Vec::new(),
        }
    }

    fn run(&mut self, state: &mut crate::state::ShellState, _: String, _: Vec<Argument>) -> String {
        let mut table = Table::new();
        self.get_files(state);
        table
            .load_preset(UTF8_FULL_CONDENSED)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(vec![
                Cell::new("Name").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Size").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Type").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Last modified").add_attribute(comfy_table::Attribute::Bold),
            ]);
        for file in &self.elements {
            if file.name.chars().nth(0) == Some('.') {
                continue;
            }
            let size: String;
            if file.size >= 1000000000 {
                size = format!("{} Gigabytes", file.size / 1000000000)
            } else if file.size >= 1000000 {
                size = format!("{} Megabytes", file.size / 1000000)
            } else if file.size >= 1000 {
                size = format!("{} Megabytes", file.size / 1000)
            } else {
                size = format!("{} Bytes", file.size)
            }
            if file.data_type == Type::File {
                table.add_row(vec![
                    Cell::new(format!("{}", file.name)).fg(Color::Green),
                    Cell::new(format!("{}", size)).fg(Color::Green),
                    Cell::new("File").fg(Color::Green),
                    Cell::new(format!(
                        "{}",
                        generate_time(file.last_modified.elapsed().unwrap())
                    ))
                    .fg(Color::Green),
                ]);
            } else {
                table.add_row(vec![
                    Cell::new(format!("{}", file.name)).fg(Color::Blue),
                    Cell::new(format!("{}", size)).fg(Color::Blue),
                    Cell::new("Folder").fg(Color::Blue),
                    Cell::new(format!(
                        "{}",
                        generate_time(file.last_modified.elapsed().unwrap())
                    ))
                    .fg(Color::Blue),
                ]);
            }
        }
        for column in table.column_iter_mut() {
            column.set_padding((0, 1)); // Removes left and right padding spaces
        }
        println!("{}", table);
        String::new()
    }
}

impl LS {
    fn get_files(&mut self, state: &mut ShellState) {
        let files_iterator = match std::fs::read_dir(state.current_directory.clone()) {
            Ok(value) => value,
            Err(_) => {
                return;
            }
        };
        for file in files_iterator {
            match file {
                Ok(file) => {
                    self.elements.push(File {
                        name: file
                            .file_name()
                            .to_str()
                            .expect("This file or operating system is corrupted in some way")
                            .to_string(),
                        size: file
                            .metadata()
                            .expect("This file or operating system is corrupted in some way")
                            .len(),
                        data_type: if file
                            .metadata()
                            .expect("This file or operating system is corrupted in some way")
                            .is_file()
                        {
                            Type::File
                        } else {
                            Type::Folder
                        },
                        last_modified: file
                            .metadata()
                            .expect("This file or operating system is corrupted in some way")
                            .modified()
                            .expect("This file or operating system is corrupted in some way"),
                    });
                }
                Err(_) => {
                    eprintln!(
                        "This file or operating system is corrupted in some way... Unable to list files"
                    );
                    exit(101);
                }
            }
        }
    }
}
fn generate_time(seconds: Duration) -> String {
    let mut output = String::new();
    if seconds.as_secs() >= 31556952 {
        output.push_str(&format!("{} Years   ", seconds.as_secs() / 31556952));
    } else if seconds.as_secs() >= 2629800 {
        output.push_str(&format!("{} Months   ", seconds.as_secs() / 2629800));
    } else if seconds.as_secs() >= 604800 {
        output.push_str(&format!("{} Weeks   ", seconds.as_secs() / 604800));
    } else if seconds.as_secs() >= 86400 {
        output.push_str(&format!("{} Days   ", seconds.as_secs() / 86400));
    } else if seconds.as_secs() >= 3600 {
        output.push_str(&format!("{} Hours   ", seconds.as_secs() / 3600));
    } else if seconds.as_secs() >= 60 {
        output.push_str(&format!("{} Minutes   ", seconds.as_secs() / 60));
    } else {
        output.push_str(&format!("{} Seconds   ", seconds.as_secs()));
    }
    output
}
