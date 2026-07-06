//! Architecture-specific kernel interfaces.
//!
//! This module conditionally compiles and re-exports bindings for the target
//! CPU architecture (e.g., `amd64`).

#[cfg(target_arch = "x86_64")] mod amd64; #[cfg(target_arch = "x86_64")] pub use amd64::*;
