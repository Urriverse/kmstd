// It's okay, it's okay, it's okay...
#![no_std]#![allow(internal_features)]#![feature(decl_macro,
core_io,string_from_utf8_lossy_owned,prelude_import)]#![allow
(unused_features)]#[allow(unused)]#[macro_use]extern crate alloc
;#[allow(unused)]#[macro_use]extern crate ketypes as ke;#[
macro_export]macro_rules!pre{()=>{#[macro_use]extern crate kstd;
#[allow(unused_imports)]#[prelude_import]use kstd::pre::*;};}pub
mod pre;#[allow(unused_imports)]#[prelude_import]pub use pre::*;

pub mod raw     ;
pub mod rt      ;
pub mod api     ;
