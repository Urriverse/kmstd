#![no_std]
#![feature(decl_macro)]

#[macro_export] macro_rules! meta { ($n:expr) => { pub macro mod_ident(){$n} } }

pub macro SYMBOL ( $( $v:vis $n:ident: $t:ty = $d:expr; )+ ) { $( #[used] #[unsafe(no_mangle)] $v static $n: $t = $d; )* }

pub mod systab;
pub mod ga;
pub mod ph;
pub mod log;

SYMBOL! { pub SYSTAB: systab::KeSysTabPtr = systab::KeSysTabPtr(core::ptr::null()); }

#[cfg(not(debug_assertions))] pub macro KeInvoke($n:ident: $($arg:expr),*) { ( unsafe { SYSTAB.0.as_ref_unchecked() }.$n )( $($arg),* ) }
#[cfg(debug_assertions)] pub macro KeInvoke($n:ident: $($arg:expr),*) { ( unsafe { SYSTAB.0.as_ref().expect("KMI fatal error") }.$n )( $($arg),* ) }

pub macro exit { () => { systab::exit(0) }, ($code:expr) => { systab::exit($code) }, }

pub macro panic_handler()
{
    #[panic_handler] fn ___km_ph(x: &core::panic::PanicInfo) -> ! { ph::k_panic(x) }
}
