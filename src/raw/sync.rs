//! Low-level synchronization primitives.
//!
//! This module re-exports kernel-level synchronization types (such as mutexes,
//! spinlocks, and so on) provided by the `ketypes` crate.

pub use ketypes::sync::*;
