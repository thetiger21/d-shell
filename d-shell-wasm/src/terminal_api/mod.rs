pub mod error;
pub mod to_event;

use std::time::Duration;

use crossterm::{
    cursor::position,
    event::poll,
    terminal::{
        WindowSize, disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, size,
        supports_keyboard_enhancement, window_size,
    },
};
use wasmtime::component::HasData;

use crate::{
    ComponentRunStates,
    terminal_api::local::runtime::{event_api, host_control::Error},
};

wasmtime::component::bindgen!({
    world: "terminal-api",
    path: "../../crossterm-wasm/wit/terminal.wit"
});

impl local::runtime::host_control::Host for ComponentRunStates {
    fn position(&mut self) -> Result<(u16, u16), Error> {
        match position() {
            Ok(data) => {
                return Ok(data);
            }
            Err(error) => return Err(Error::serialise(error)),
        }
    }

    fn poll(&mut self, timeout: u64) -> Result<bool, Error> {
        match poll(Duration::from_nanos(timeout)) {
            Ok(data) => return Ok(data),
            Err(error) => return Err(Error::serialise(error)),
        }
    }

    fn read(&mut self) -> Result<event_api::Event, Error> {
        match crossterm::event::read() {
            Ok(event) => return Ok(event.into()),
            Err(error) => return Err(Error::serialise(error)),
        };
    }

    fn supports_keyboard_enhancement(&mut self) -> Result<bool, Error> {
        match supports_keyboard_enhancement() {
            Ok(data) => return Ok(data),
            Err(error) => return Err(Error::serialise(error)),
        }
    }

    fn is_raw_mode_enabled(&mut self) -> Result<bool, Error> {
        match is_raw_mode_enabled() {
            Ok(data) => return Ok(data),
            Err(error) => return Err(Error::serialise(error)),
        }
    }

    fn enable_raw_mode(&mut self) -> Result<(), Error> {
        match enable_raw_mode() {
            Ok(_) => Ok(()),
            Err(error) => Err(Error::serialise(error)),
        }
    }

    fn disable_raw_mode(&mut self) -> Result<(), Error> {
        match disable_raw_mode() {
            Ok(_) => Ok(()),
            Err(error) => Err(Error::serialise(error)),
        }
    }

    fn size(&mut self) -> Result<(u16, u16), Error> {
        match size() {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::serialise(error)),
        }
    }

    fn get_window_size(&mut self) -> Result<local::runtime::host_control::WindowSize, Error> {
        match window_size() {
            Ok(window_size) => {
                return Ok(local::runtime::host_control::WindowSize {
                    rows: window_size.rows,
                    columns: window_size.columns,
                    width: window_size.width,
                    height: window_size.height,
                });
            }
            Err(error) => Err(Error::serialise(error)),
        }
    }
}

impl HasData for ComponentRunStates {
    type Data<'a> = &'a mut ComponentRunStates;
}
