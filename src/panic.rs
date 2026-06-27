#[cfg(not(test))]
#[panic_handler]
fn _panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
