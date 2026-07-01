use crate::*; use ketypes::*;

Import! {
    pub fn MemAlloc(layout: core::alloc::Layout) -> *mut u8 where kernel 0.0 {
        let _ = layout;
        error!("MemAlloc not provided");
        return 0 as _;
    }
}

Import! {
    pub fn MemFree(ptr: *mut u8, layout: core::alloc::Layout) where kernel 0.0 {
        let _ = (ptr, layout);
        error!("MemFree not provided");
    }
}
