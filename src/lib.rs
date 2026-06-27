#![no_std]
#![feature(decl_macro)]

pub mod systab;
pub mod ga;

#[unsafe(no_mangle)]
pub static SYSTAB: systab::KeSysTabPtr = systab::KeSysTabPtr(core::ptr::null());

pub macro KeInvoke($n:ident: $($arg:expr),*) {
    (
        unsafe {
            SYSTAB.0.as_ref_unchecked()
        }.$n
    )
    ( $($arg),* )
}

ga::global_allocator!{}
