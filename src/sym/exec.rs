use ketypes::*; use crate::*;

Import! {
    pub fn ExecPanic(info: &core::panic::PanicInfo) -> ! where kernel 0.0 {
        panic_msg!("ExecPanic not provided");
        loop{core::hint::spin_loop()}
    }
}

Import! {
    pub fn ExecExit(code: i32) -> ! where kernel 0.0 {
        panic_msg!("ExecExit not provided, tried to exit with code = {}", code);
        loop{core::hint::spin_loop()}
    }
}

// Import! {
//     pub fn ExecYield() where kernel 0.0 {
//         error!("ExecYield not provided");
//     }
// }

Import! {
    pub fn ExecSleep() where kernel 0.0 {
        error!("ExecYield not provided");
    }
}
