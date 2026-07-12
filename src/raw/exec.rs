//! Task and process execution control.
//!
//! This module provides raw bindings for task lifecycle management, including
//! spawning new tasks (from closures or function pointers), yielding the CPU,
//! sleeping, waiting, and terminating execution.

use alloc::boxed::{Box, ThinBox};

#[repr(transparent)]
pub struct WaitQueue
(
    &'static()
);

pub type TaskId = u64;

pub type ProcId = u64;

pub type Priority = i32;

pub type ExitCode = i32;

#[non_exhaustive]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sig
{
    TERM =   1, // termination
    INTR =   2, // interrupt
    EXCP =   4,
    USR0 =   8, // user (0)
    USR1 =  16, // user (1)
    USR2 =  32, // user (2)
    USR3 =  64, // user (3)
    RSVD = 128, // reserved (all after this bit MUST BE zero for future extention)
}

#[repr(C)]
pub struct IntInfo
{
    pub sig: Sig,
    pub data: u64,
    pub frame: ThinBox<crate::raw::TrapFrame>,
}

/// Signal handler. If returns `true`, kernel will kill current **process**.
/// 
/// First argument is signal, second is additional info (e. g., pointer to TrapFrame on exception).
pub type SigHdl = fn(&IntInfo) -> bool;

#[allow(non_snake_case)]
pub fn ExecSpawnClosure<F>(f: F, pri: Priority, name: String, cpu: Option<usize>)
where
    F: FnOnce() + Send + 'static,
{
    let boxed = Box::new(f);
    let arg = Box::into_raw(boxed) as usize;

    fn trampoline<F: FnOnce() + Send + 'static>(arg: usize)
    {
        let f = unsafe { Box::from_raw(arg as *mut F) };
        f();
        ExecExit(0);
    }

    ExecSpawnArg(trampoline::<F>, arg, pri, name, cpu, false);
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn ExecYield()
{
    unsafe
    {
        core::arch::asm!
        {
            "int 33"
        }
    }
}

Import!
{
    pub fn ExecExit(code: i32) -> !
    where kernel 0.1;

    pub fn ExecPanic(info: &core::panic::PanicInfo) -> !
    where kernel 0.1;

    pub fn ExecSetDeadline(ms: u64) -> Result<(), ()>
    where kernel 0.1;

    pub fn ExecSpawnTask(f: fn(), pri: Priority, name: String, cpu: Option<usize>, npid: bool)
    where kernel 0.1;

    pub fn ExecSpawnArg(f: fn(usize), arg: usize, pri: Priority, name: String, cpu: Option<usize>, npid: bool)
    where kernel 0.1;

    pub fn ExecTaskId() -> TaskId
    where kernel 0.1;

    pub fn ExecRoot() -> String
    where kernel 0.1;

    pub fn ExecSleep(wq: WaitQueue)
    where kernel 0.1;

    pub fn ExecWakeup(wq: WaitQueue)
    where kernel 0.1;

    pub fn ExecWait(child_id: TaskId) -> i32
    where kernel 0.1;

    pub fn ExecProcess() -> Option<ProcId>
    where kernel 0.1;

    pub fn ExecPwd() -> Option<String>
    where kernel 0.1;

    pub fn ExecSleepMs(n: u64)
    where kernel 0.1;

    pub fn ExecKillTaskById(tid: TaskId)
    where kernel 0.1;

    pub fn ExecKillProcById(pid: ProcId)
    where kernel 0.1;

    pub fn ExecSetSigHandler(sh: SigHdl)
    where kernel 0.1;

    pub fn ExecInterruptTask(tid: TaskId, sig: Sig, data: u64)
    where kernel 0.1;
}
