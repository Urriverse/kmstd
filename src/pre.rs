#![allow(unused_macros)]

pub use core::prelude::v1::*;
pub use alloc::{format, vec, vec::Vec, collections::*};
pub use crate::raw::sync;
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
