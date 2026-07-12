//! The root of the kernel-specific standard library.
//!
//! This crate provides a `no_std` environment tailored for a Kernel,
//! offering both low-level system bindings and high-level safe abstractions.
//!
//! # Modules
//!
//! - [`raw`]: Low-level, direct kernel ABI and system call bindings.
//! - [`rt`]: Runtime support, including the global allocator and panic handler.
//! - [`api`]: High-level, safe APIs resembling the Rust standard library (`std`).

#![no_std]

#![allow(internal_features)]
#![allow(unused_features)]

#![feature
(
    decl_macro                  ,
    core_io                     ,
    string_from_utf8_lossy_owned,
    prelude_import              ,
    doc_cfg                     ,
    custom_inner_attributes     ,
    proc_macro_hygiene          ,
    thin_box                    ,
)]

#[allow(unused)]
#[macro_use]
extern crate alloc;

#[allow(unused)]
#[macro_use]
extern crate ketypes;

#[macro_export]
macro_rules! pre
{
    [] =>
    {
        #[macro_use]
        extern crate kstd as __kstd;
        
        #[allow(unused_imports)]
        #[prelude_import]
        use kstd::pre::*;
    }
}

pub mod pre;

#[allow(unused_imports)]
#[prelude_import]
pub use pre::*;

pub mod raw;
pub mod rt;
pub mod api;
