fn placeholder0(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop()
    }
}

crate::SYMBOL! {
    pub k_panic: fn(&core::panic::PanicInfo) -> ! = placeholder0;
}
