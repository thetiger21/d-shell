pub mod parse;
pub mod run;

use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::*;
use wasmtime_wasi::p2::bindings::sync::Command;
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxView, WasiView};

pub struct ComponentRunStates {
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

pub struct WasmRuntime {
    engine: Engine,
    linker: Linker<ComponentRunStates>,
}

pub enum ProgamState {
    Success,
    Failed,
}

impl WasmRuntime {
    pub fn new() -> Result<Self> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        Ok(Self { engine, linker })
    }

    pub fn execute(
        &mut self,
        args: &Vec<String>,
        program_path: String,
        current_directory: String,
        file_name: String,
    ) -> Result<ProgamState> {
        let wasi_ctx = WasiCtx::builder()
            .inherit_stdio()
            .inherit_env()
            .inherit_network()
            .args(&args)
            .preopened_dir(
                format!("{}/", current_directory),
                "/",
                DirPerms::all(),
                FilePerms::all(),
            )?
            .build();

        let state = ComponentRunStates {
            wasi_ctx,
            resource_table: ResourceTable::new(),
        };

        let mut store = Store::new(&self.engine, state);

        let path = format!("{}/{}/bin", program_path, file_name);
        let component = Component::from_file(&self.engine, path)?;

        let command = Command::instantiate(&mut store, &component, &self.linker)?;
        let program_result = command.wasi_cli_run().call_run(&mut store)?;
        if program_result.is_err() {
            return Ok(ProgamState::Failed);
        }
        Ok(ProgamState::Success)
    }
}
