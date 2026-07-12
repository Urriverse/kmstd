//! High-level, safe API abstractions.
//!
//! This module provides a user-friendly, safe interface to the underlying kernel
//! services, closely mirroring the design of the Rust standard library (`std`).
//! It includes modules for file system operations, environment variables,
//! task management and other things unsefull in kernel module development.

pub mod path    ;
pub mod fs      ;
pub mod env     ;
pub mod task    ;

pub use crate::raw;
pub use core::
{
    io,
    result,
    fmt,
    range,
    ops,
    option,
    panic,
    any,
    array,
    ascii,
    slice,
    str,
    hash,
    hint,
    cell,
    cmp,
    borrow,
    num,
    marker,
    mem,
    ptr
};
