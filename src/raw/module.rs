//! Dynamic module loading and execution.
//!
//! This module provides raw bindings for loading binary modules into memory,
//! linking them, and spawning them as new tasks.

use alloc::string::String;

use super::*;

#[repr(transparent)] #[derive(Clone, Copy)] pub struct Module(usize);

Import! {
    pub fn ModLoad(bytes: &'static [u8]) -> Result<Module, String> where kernel 0.1;

    pub fn ModLink(module: Module) -> Result<(), String> where kernel 0.1;

    pub fn ModRun(module: Module) -> Result<TaskId, String> where kernel 0.1;
}
