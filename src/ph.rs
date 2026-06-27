#[cfg(not(test))] #[panic_handler] pub fn phdl(pi: &core::panic::PanicInfo) -> ! { crate::KeInvoke!(panic: pi) }
