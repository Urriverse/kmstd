pub macro panic_handler() {
    #[panic_handler]
    fn ___km_ph(x: &core::panic::PanicInfo) -> ! {
        crate::sym::k_panic(x)
    }
}

pub macro exit {
    () => {
        crate::sym::k_exit(0)
    },
    ($code:expr) => {
        crate::sym::k_exit($code)
    },
}
