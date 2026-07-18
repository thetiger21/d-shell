use crate::{state::ShellState, token_types::Argument};

pub mod cat;
pub mod cd;
pub mod clear;
pub mod echo;
pub mod exit;
pub mod help;
pub mod ls;
pub mod mkdir;
pub mod rm;
pub mod run;
pub mod touch;
pub mod write;
pub mod zip;

pub trait ShellCommand {
    fn new() -> Self
    where
        Self: Sized;
    fn run(&mut self, state: &mut ShellState, input: String, arguments: Vec<Argument>) -> String;
}
