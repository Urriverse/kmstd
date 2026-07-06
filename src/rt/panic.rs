//! Panic handler implementation.
//!
//! This module defines the `#[panic_handler]` for the crate, routing Rust panics
//! to the kernel's panic system call to safely terminate the faulting task.

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(x: &core::panic::PanicInfo) -> ! {
    crate::raw::ExecPanic(x)
}
