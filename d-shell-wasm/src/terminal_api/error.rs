use std::io::{Error, ErrorKind};

use crate::terminal_api::local;

impl local::runtime::error::Error {
    pub fn serialise(error: Error) -> Self {
        let error_kind = match error.kind() {
            ErrorKind::Interrupted => local::runtime::error::ErrorKind::Interrupted,
            ErrorKind::TimedOut => local::runtime::error::ErrorKind::TimedOut,
            ErrorKind::WouldBlock => local::runtime::error::ErrorKind::WouldBlock,
            _ => local::runtime::error::ErrorKind::Other,
        };
        Self {
            kind: error_kind,
            message: format!("{}", error),
        }
    }
}
