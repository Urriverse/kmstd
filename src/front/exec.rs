pub type WaitQueue = ketypes::Nutex<usize>;

pub type TaskId = u64;

pub type Priority = i32;

pub type ExitCode = i32;

Import! {
    pub fn ExecExit(code: i32) -> ! where kernel 0.1;
    pub fn ExecPanic(info: &core::panic::PanicInfo) -> ! where kernel 0.1;
    pub fn ExecSetDeadline(ms: u64) -> Result<(), ()> where kernel 0.1;
}

#[allow(non_snake_case)] pub fn ExecYield() { unsafe { core::arch::asm! { "int 33" } } }
