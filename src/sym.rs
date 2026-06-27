use crate::{stub, log::AttLvl};
use core::fmt::Arguments;

crate::util::SYMBOL! {
    pub k_panic : fn (&core::panic::PanicInfo) -> !                         = stub::panic   ;

    pub k_exit  : fn (i32) -> !                                             = stub::exit    ;

    pub k_alloc : fn (core::alloc::Layout) -> *mut u8                       = stub::alloc   ;

    pub k_free  : fn (*mut u8, core::alloc::Layout)                         = stub::free    ;

    pub k_log   : fn (AttLvl,&'static str,&'static str,u32,Arguments<'_>)   = stub::log     ;
}
