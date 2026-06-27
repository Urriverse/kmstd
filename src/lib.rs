#![no_std]
#![feature(decl_macro)]

#[allow(unused)] #[macro_use] pub extern crate alloc;

pub mod entry;
pub mod panic;
pub mod gall;
pub mod kst;

pub macro allocator() {
    #[global_allocator]
    pub static GALL: $crate::gall::Gall = $crate::gall::Gall;
}
