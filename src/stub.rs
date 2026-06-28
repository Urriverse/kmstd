pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop()
    }
}

pub fn exit(_: i32) -> ! {
    loop {
        core::hint::spin_loop()
    }
}

pub fn alloc(_: core::alloc::Layout) -> *mut u8 {
    0 as *mut u8
}

pub fn free(_: *mut u8, _: core::alloc::Layout) {
    /* nothing */
}

pub fn log(_: ketypes::mon::lvl::AttLvl, _: &'static str, _: &'static str, _: u32, _: core::fmt::Arguments<'_>) {
    /* nothing */
}
