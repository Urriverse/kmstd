#[cfg(not(test))]
#[panic_handler]
fn panic_handler(x: &core::panic::PanicInfo) -> ! {
    crate::raw::ExecPanic(x)
}
