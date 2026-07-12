//! Low-level kernel ABI and direct system call bindings.
//!
//! This module contains the raw, mostly unsafe, interfaces to the underlying kernel.
//! It provides direct access to memory management, execution, file systems,
//! virtual memory, architecture-specific features, and synchronization primitives.
//!
//! Most users should prefer the safe abstractions provided in the [`crate::api`] module
//! while low-level capabilities not strictly required.

insmod!
{
    pub arch    pub,
    pub event   pub,
    pub exec    pub,
    pub gum     pub,
    pub mem     pub,
    pub module  pub,
    pub mon     pub,
    pub sync    pub,
    pub vfs     pub,
    pub vm      pub,
}
