//! The custom prelude for the kernel standard library.
//!
//! This module re-exports commonly used items from `core`, `alloc`, and the kernel's
//! raw synchronization primitives. It also provides essential macros for logging
//! and defining module entry points.
//! 
//! There are:
//! 
//! - [`core`] and [`alloc`] preludes;
//! - [`sync`] module with sychronization primitives;
//! - [`std`] module with high-level API
//! 
//! > **NOTE**: all items declared above are imported in each your module and submodule
//! > for convenience. If you don't like that, you can write (*example below*) "ecrate"
//! > directive and use items by full path.
//! 
//! ```rust,ignore
//! // "ecrate" directive:
//! extern crate kstd;
//! 
//! // prelude activation:
//! #![allow(internal_features)] #![feature(prelude_import)] kstd::pre![];
//! ```

#![allow(unused_macros)]

pub use core::prelude::v1::*;
pub use alloc::{format, vec, vec::Vec, collections::*};
pub use crate::raw::sync;
pub use proc::status;
pub use crate::api as std;  // why not

macro m() {
    concat!(
        env!(
            "CARGO_PKG_NAME"
        ),
        "::",
        module_path!()
    )
}

macro log( $l:ident $($arg:tt)+ ) {
    crate::raw::MonLog(
        crate::raw::AttLvl::$l,
        m!(), file!(), line!(),
        format_args!($($arg)+)
    );
}

pub macro trace( $($arg:tt)+ ) { log!( Trace $($arg)+ ); }
pub macro debug( $($arg:tt)+ ) { log!( Debug $($arg)+ ); }
pub macro  info( $($arg:tt)+ ) { log!(  Info $($arg)+ ); }
pub macro  warn( $($arg:tt)+ ) { log!(  Warn $($arg)+ ); }
pub macro error( $($arg:tt)+ ) { log!( Error $($arg)+ ); }
pub macro fatal( $($arg:tt)+ ) { log!( Panic $($arg)+ ); }

pub macro println {
    ( ) => { info!(""); },
    ( $($arg:tt)+ ) => { info!( $($arg)+ ) },
}

pub macro entry( mod $n:literal ; $( $b:tt )* ) {
    crate::raw::SYMBOL! { pub MODNAME:&'static str=$n; }
    
    #[unsafe(no_mangle)] pub extern "C" fn _start() { $($b)* }
}
