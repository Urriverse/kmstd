//! Inspection of the task's environment.
//!
//! This module provides functions to query the execution environment of the
//! current task, such as retrieving the current working directory.

#![status(incomplete)]

pub fn current_dir() -> super::fs::Result<super::fs::Path> {
    match crate::raw::ExecPwd() {
        Some(p) => Ok(p),
        None => Err(core::io::Error::new(core::io::ErrorKind::InvalidData, "Unknown task"))
    }
}
