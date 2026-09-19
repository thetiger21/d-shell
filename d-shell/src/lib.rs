pub mod command_processor;
pub mod input;
pub mod script_parsing;
pub mod state;
pub mod token_types;

use std::io::{self};

use d_shell_wasm::WasmRuntime;

use crate::{input::get_input::get_input, state::ShellState};

pub fn run_shell() -> io::Result<String> {
    let mut shell_state = ShellState::new();
    let wasm_runtime = WasmRuntime::new().unwrap();

    get_input(&mut shell_state, wasm_runtime)
}
