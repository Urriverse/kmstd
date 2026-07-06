#![no_std]
#![feature(decl_macro)]
#![feature(core_io)]
#![feature(string_from_utf8_lossy_owned)]

extern crate alloc;
#[macro_use] extern crate ketypes;

pub mod log;
pub mod util;
pub mod macros;
pub mod ga;
pub mod front;
pub mod sugar;
pub mod pre;

pub use macros::*;
pub use util::*;
pub use log::*;

pub use front::*;
pub use sugar::*;

pub use pre::*;

#[macro_export]
macro_rules! pre {
    () => {
        #[macro_use] extern crate kstd;
        #[prelude_import]
        use kstd::pre::*;
    };
}
