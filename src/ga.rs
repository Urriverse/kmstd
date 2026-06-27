use crate::sym;

struct __GA;

#[used]
#[global_allocator]
static __GA_INSTANCE: __GA = __GA;

unsafe impl core::alloc::GlobalAlloc for __GA {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        sym::k_alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        sym::k_free(ptr, layout)
    }
}
