use alloc::string::String;

use crate::*;

#[repr(transparent)] #[derive(Clone, Copy)] pub struct Module(usize);

Import! {
    pub fn ModLoad(bytes: &'static [u8]) -> Result<Module, String> where kernel 0.1;

    pub fn ModLink(module: Module) -> Result<(), String> where kernel 0.1;

    pub fn ModRun(module: Module) -> Result<TaskId, String> where kernel 0.1;
}
