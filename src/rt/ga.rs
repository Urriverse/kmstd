//! Global allocator implementation.
//!
//! This module implements the [`core::alloc::GlobalAlloc`] trait, bridging Rust's
//! heap allocation requests (`alloc::vec!`, `Box`, etc.) to the kernel's raw
//! memory allocation system calls.

use crate::*;

struct __GA;

#[used]
#[global_allocator]
static __GA_INSTANCE: __GA = __GA;

unsafe impl core::alloc::GlobalAlloc for __GA
{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8
    {
        raw::MemAlloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout)
    {
        raw::MemFree(ptr, layout)
    }
}
