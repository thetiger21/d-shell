use crate::{state::ShellState, wasm::WasmRuntime};

impl ShellState {
    pub fn run_program(&mut self, input: &String, wasm_runtime: &mut WasmRuntime) {
        let mut tokens = self.program_parse(&input);
        let path_directory = &self.clone().get_programs_directory();
        let current_directory = self.current_directory.clone();
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
}
