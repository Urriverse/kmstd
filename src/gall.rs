use alloc::alloc::{GlobalAlloc, Layout};


pub struct Gall;

unsafe impl GlobalAlloc for Gall {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        0 as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //
    }
}
