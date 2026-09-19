use crate::{WasmRuntime, parse::program_parse};

pub struct WasmProgramRunningConfiguration {
    pub program_directory: String,
    pub current_directory: String,
    pub input: String,
}

pub fn run_program(
    wasm_program_running_configuration: WasmProgramRunningConfiguration,
    wasm_runtime: &mut WasmRuntime,
) {
    let mut tokens = program_parse(&wasm_program_running_configuration.input);
    let path_directory = wasm_program_running_configuration.program_directory;
    let current_directory = wasm_program_running_configuration.current_directory;
    let tokens_new = tokens.clone();
    let binding = tokens_new.get(0);
    let program_name = match binding {
        Some(data) => data,
        None => {
            eprintln!("Nothing exists in position 0");
            return ();
        }
    };
    tokens.remove(0);

    match wasm_runtime.execute(
        &tokens,
        path_directory.clone(),
        current_directory,
        program_name.to_string(),
    ) {
        Ok(_) => (),
        Err(error_msg) => {
            eprintln!("{}", error_msg);
        }
    }
}
