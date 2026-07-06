//! A module for working with tasks and processes.
//!
//! This module is mostly concerned with spawning and interacting with child
//! tasks and processes, but it also provides [`abort`] and [`exit`] for
//! terminating the current task.

/// OS-assigned process identifier associated with process.
#[status(stable)]
pub use crate::raw::ProcId as PID;

/// OS-assigned process identifier associated with task.
#[status(stable)]
pub use crate::raw::TaskId as TID;

/// Terminates the current task with the specified exit code.
///
/// This function will never return and will immediately terminate the current
/// task. The exit code is passed through to the waiting task or to the reaper.
///
/// Note that because this function never returns, and that it terminates the
/// task, no destructors on the current stack or any other thread's stack
/// will be run. If a clean shutdown is needed it is recommended to only call
/// this function at a known point where there are no more destructors left
/// to run.
#[status(stable)]
pub fn exit(code: i32) -> ! {
    crate::raw::ExecExit(code)
}

/// Terminates the task in an abnormal fashion.
///
/// The function will never return and will immediately terminate the current
/// task in a platform specific "abnormal" manner. As a consequence,
/// no destructors on the current stack or any other thread's stack
/// will be run, Rust IO buffers (eg, from `BufWriter`) will not be flushed,
/// and C stdio buffers will (on most platforms) not be flushed.
///
/// This is in contrast to the default behavior of [`panic!`] which unwinds
/// the current thread's stack and calls all destructors.
///
/// If a clean shutdown is needed it is recommended to only call
/// this function at a known point where there are no more destructors left
/// to run.
#[status(stable)]
pub fn abort() -> ! {
    crate::raw::ExecExit(-1)
}

/// Returns the OS-assigned process identifier associated with this process.
#[status(stable)]
pub fn pid() -> PID {
    crate::raw::ExecProcess().unwrap_or(0)
}

/// Returns the OS-assigned process identifier associated with this task.
#[status(stable)]
pub fn task_id() -> TID {
    crate::raw::ExecTaskId()
}
