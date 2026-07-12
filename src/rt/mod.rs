//! Kernel runtime support.
//!
//! This module contains essential runtime components required for the standard
//! library to function in a `no_std` environment, such as the global allocator
//! and the panic handler.

pub mod ga;
pub mod panic;
pub mod start;
