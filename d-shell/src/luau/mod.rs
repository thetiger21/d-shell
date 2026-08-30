pub mod graphics;

use std::fs;

use mlua::{Lua, Result};

use crate::luau::graphics::init_graphics_env;

pub fn run_lua(file_name: &str) -> Result<String> {
    let luau = Lua::new();
    init_graphics_env(&luau);
    let contents = fs::read_to_string(file_name).map_err(mlua::Error::external)?;
    let result = luau.load(contents).eval::<String>();
    result
}
