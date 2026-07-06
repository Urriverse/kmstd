use alloc::{boxed::Box, string::String};

#[repr(transparent)] pub struct WaitQueue(&'static());

pub type TaskId = u64;

pub type ProcId = u32;

pub type Priority = i32;

pub type ExitCode = i32;

#[allow(non_snake_case)]
pub fn ExecSpawnClosure<F>(
    closure: F,
    priority: Priority,
    name: alloc::string::String,
    cpu_affinity: Option<usize>,
)
where
    F: FnOnce() + Send + 'static,
{
    let boxed = Box::new(closure);
    let arg = Box::into_raw(boxed) as usize;

    fn trampoline<F: FnOnce() + Send + 'static>(arg: usize) {
        let closure = unsafe { Box::from_raw(arg as *mut F) };
        closure();
        ExecExit(0);
    }

    ExecSpawnArg(trampoline::<F>, arg, priority, name, cpu_affinity, false);
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn ExecYield() {
    unsafe {
        core::arch::asm! {
            "int 33"
        }
    }
}

Import! {
    pub fn ExecExit(code: i32) -> ! where kernel 0.1;
    pub fn ExecPanic(info: &core::panic::PanicInfo) -> ! where kernel 0.1;
    pub fn ExecSetDeadline(ms: u64) -> Result<(), ()> where kernel 0.1;
    pub fn ExecSpawnTask(entry: fn(), priority: Priority, name: String, affn: Option<usize>, npid: bool) where kernel 0.1;
    pub fn ExecSpawnArg(entry: fn(usize), arg: usize, priority: Priority, name: String, affn: Option<usize>, npid: bool) where kernel 0.1;
    pub fn ExecTaskId() -> TaskId where kernel 0.1;
    pub fn ExecRoot() -> String where kernel 0.1;
    pub fn ExecSleep(wq: WaitQueue) where kernel 0.1;
    pub fn ExecWakeup(wq: WaitQueue) where kernel 0.1;
    pub fn ExecWait(child_id: TaskId) -> i32 where kernel 0.1;
    pub fn ExecProcess() -> Option<ProcId> where kernel 0.1;
}
