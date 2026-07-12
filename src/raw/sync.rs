//! Low-level synchronization primitives.
//!
//! This module re-exports kernel-level synchronization types (such as mutexes,
//! spinlocks, and so on) provided by the `ketypes` crate and synchronous smart
//! pointers from `alloc`

pub use ketypes::sync::*;
pub use alloc::sync::*;
