#![no_std]
#![feature(decl_macro)]

pub mod systab;
pub mod ga;
pub mod ph;
pub mod log;

#[used] #[unsafe(no_mangle)] pub static SYSTAB: systab::KeSysTabPtr = systab::KeSysTabPtr(core::ptr::null());

#[cfg(not(debug_assertions))] pub macro KeInvoke($n:ident: $($arg:expr),*) { ( unsafe { SYSTAB.0.as_ref_unchecked() }.$n )( $($arg),* ) }
#[cfg(debug_assertions)] pub macro KeInvoke($n:ident: $($arg:expr),*) { ( unsafe { SYSTAB.0.as_ref().expect("KMI fatal error") }.$n )( $($arg),* ) }

#[macro_export]
macro_rules! meta {
    ($n:expr) => {
        pub macro mod_ident(){$n}
    };
}
