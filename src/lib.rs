#![no_std]
#![feature(decl_macro)]

pub mod systab;

pub macro meta() {
    #[unsafe(no_mangle)]
    pub static SYSTAB: $crate::systab::KeSysTabPtr = $crate::systab::KeSysTabPtr(core::ptr::null());
}
