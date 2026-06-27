struct GA;

unsafe impl core::alloc::GlobalAlloc for GA {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        crate::KeInvoke!(alloc: layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        crate::KeInvoke!(free: ptr, layout)
    }
}

#[global_allocator] static ___GAI: GA = GA;
