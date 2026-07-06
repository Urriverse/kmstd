//! Low-level kernel ABI and direct system call bindings.
//!
//! This module contains the raw, mostly unsafe, interfaces to the underlying kernel.
//! It provides direct access to memory management, execution, file systems,
//! virtual memory, architecture-specific features, and synchronization primitives.
//!
//! Most users should prefer the safe abstractions provided in the [`crate::api`] module
//! while low-level capabilities not strictly required.

pub macro SYMBOL( $( $v:vis $n:ident : $t:ty = $d:expr; )+ ) {
    $(
        #[used]
        #[unsafe(no_mangle)]
        $v static $n: $t = $d;
    )*
}

pub mod arch    ; pub use arch  ::*;
pub mod event   ; pub use event ::*;
pub mod exec    ; pub use exec  ::*;
pub mod gum     ; pub use gum   ::*;
pub mod mem     ; pub use mem   ::*;
pub mod module  ; pub use module::*;
pub mod mon     ; pub use mon   ::*;
pub mod sync    ; pub use sync  ::*;
pub mod vfs     ; pub use vfs   ::*;
pub mod vm      ; pub use vm    ::*;
