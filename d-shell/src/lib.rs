pub mod command_processor;
pub mod input;
pub mod luau;
pub mod script_parsing;
pub mod state;
pub mod token_types;
pub mod wasm;

use std::io::{self};

use crate::{input::get_input::get_input, state::ShellState, wasm::WasmRuntime};

pub fn run_shell() -> io::Result<String> {
    let mut shell_state = ShellState::new();
    let wasm_runtime = WasmRuntime::new().unwrap();

    get_input(&mut shell_state, wasm_runtime)
}
