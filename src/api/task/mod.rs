//! A module for working with tasks and processes.
//!
//! This module is mostly concerned with spawning and interacting with child
//! tasks and processes, but it also provides [`abort`] and [`exit`] for
//! terminating the current task.
//!
//! # Signals
//!
//! The module provides a signal system similar to POSIX signals. You can:
//!
//! - Send signals to tasks using [`send_signal`]
//! - Send signals to the current task using [`raise`]
//! - Set a global signal handler using [`set_signal_handler`]
//!
//! # Spawning tasks
//!
//! ```rust,ignore
//!
//! let handle = std::task::spawn(|| {
//!     println!("Hello from a new task!");
//!     42
//! });
//!
//! let result = handle.join().unwrap();
//! assert_eq!(result, 42);
//! ```

use alloc::string::String;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use core::time::Duration;

use crate::raw::*;

/// Kernel-assigned process identifier associated with process.
#[status(stable)]
pub use crate::raw::ProcId as PID;

/// Kernel-assigned process identifier associated with task.
#[status(stable)]
pub use crate::raw::TaskId as TID;

/// Signal types that can be sent to tasks.
///
/// Signals are asynchronous notifications sent to tasks. They can be used
/// for inter-task communication, interrupt handling, or requesting graceful
/// termination.
///
/// # Available signals
///
/// - [`TERM`]: Termination request (graceful shutdown)
/// - [`INTR`]: Interrupt signal
/// - [`EXCP`]: CPU Exception signal
/// - [`USR0`]-[`USR3`]: User-defined signals
#[status(stable)]
pub use crate::raw::Sig;

/// Information about a received signal.
///
/// This structure is passed to signal handlers and contains:
/// - The signal type that was received
/// - Additional data associated with the signal
/// - A pointer to the trap frame (for exception signals)
#[status(stable)]
pub use crate::raw::IntInfo;

/// Signal handler function type.
///
/// A signal handler receives a reference to [`IntInfo`] and returns a boolean:
/// - `true`: The kernel should kill the current **process** after the handler returns
/// - `false`: Normal execution continues after the handler returns
///
/// # Examples
///
/// ```rust,ignore
/// use std::task::{Sig, IntInfo};
///
/// fn my_handler(info: &IntInfo) -> bool {
///     match info.sig {
///         Sig::TERM => {
///             println!("Received termination signal, cleaning up...");
///             false // Don't kill the process, let main() handle cleanup
///         }
///         Sig::USR0 => {
///             println!("User signal received with data: {}", info.data);
///             false
///         }
///         _ => true // Kill process for unknown signals
///     }
/// }
///
/// std::task::set_signal_handler(my_handler);
/// ```
#[status(stable)]
pub use crate::raw::SigHdl;

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
pub fn exit(code: i32) -> !
{
    ExecExit(code)
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
pub fn abort() -> !
{
    ExecExit(-1)
}

/// Returns the kernel-assigned process identifier associated with this process.
#[status(stable)]
pub fn pid() -> PID
{
    ExecProcess().unwrap_or(0)
}

/// Returns the kernel-assigned process identifier associated with this task.
#[status(stable)]
pub fn task_id() -> TID
{
    ExecTaskId()
}

/// Cooperatively gives up a timeslice to the kernel scheduler.
///
/// This is useful when implementing busy-waiting loops or when you want to
/// hint to the scheduler that the current task is willing to give up its
/// timeslice.
#[status(stable)]
pub fn yield_now()
{
    ExecYield();
}

/// Puts the current task to sleep for at least the specified amount of time.
///
/// This function will block the current task for the specified duration.
/// The task may sleep longer than specified if the system is busy.
#[status(stable)]
pub fn sleep(duration: Duration)
{
    ExecSleepMs(duration.as_millis() as u64);
}

/// Sets the global signal handler for the current process.
///
/// The handler will be called whenever any task in the current process
/// receives a signal. The handler can inspect the signal type and data,
/// and return `true` to request that the kernel kill the entire process,
/// or `false` to continue normal execution.
///
/// # Examples
///
/// ```rust,ignore
/// use std::task::{Sig, IntInfo};
///
/// fn signal_handler(info: &IntInfo) -> bool {
///     println!("Received signal: {:?}", info.sig);
///     match info.sig {
///         Sig::TERM => {
///             // Graceful shutdown - don't kill process immediately
///             false
///         }
///         Sig::EXCP => true // kill process on CPU exception
///         _ => false // ignore other
///     }
/// }
///
/// std::task::set_signal_handler(signal_handler);
/// ```
#[status(stable)]
pub fn set_signal_handler(handler: SigHdl)
{
    ExecSetSigHandler(handler);
}

/// Sends a signal to a specific task.
///
/// This function asynchronously delivers the specified signal to the task
/// identified by `tid`. The task's signal handler (if set) will be invoked
/// with the provided signal and data.
///
/// # Parameters
///
/// - `tid`: The task ID to send the signal to
/// - `sig`: The signal type to send
/// - `data`: Additional data to pass to the signal handler
///
/// # Examples
///
/// ```rust,ignore
/// use std::task::{Sig, Duration};
///
/// let handle = std::task::spawn(|| {
///     // Long-running task
///     loop {
///         std::task::sleep(Duration::from_secs(1));
///     }
/// });
///
/// let tid = handle.task_id();
///
/// // Send a termination signal
/// std::task::send_signal(tid, Sig::TERM, 0);
/// ```
#[status(stable)]
pub fn send_signal(tid: TID, sig: Sig, data: u64)
{
    ExecInterruptTask(tid, sig, data);
}

/// Sends a signal to the current task.
///
/// This is a convenience function equivalent to `send_signal(task_id(), sig, data)`.
///
/// # Examples
///
/// ```rust,ignore
/// use std::task::Sig;
///
/// // Send a user signal to self
/// std::task::raise(Sig::USR0, 42);
/// ```
#[status(stable)]
pub fn raise(sig: Sig, data: u64)
{
    ExecInterruptTask(task_id(), sig, data);
}

/// Sends an interrupt signal to a specific task.
///
/// This is a convenience wrapper around [`send_signal`] that sends the
/// [`Sig::INTR`] signal with `data = 0`.
///
/// [`send_signal`]: fn.send_signal.html
/// [`Sig::INTR`]: enum.Sig.html#variant.INTR
#[status(stable)]
pub fn interrupt(tid: TID)
{
    send_signal(tid, Sig::INTR, 0);
}

/// Sends a termination signal to a specific task.
///
/// This is a convenience wrapper around [`send_signal`] that sends the
/// [`Sig::TERM`] signal with `data = 0`. This is a graceful termination
/// request that allows the task's signal handler to perform cleanup.
///
/// For immediate termination without invoking signal handlers, use [`kill`].
///
/// [`send_signal`]: fn.send_signal.html
/// [`Sig::TERM`]: enum.Sig.html#variant.TERM
/// [`kill`]: fn.kill.html
#[status(stable)]
pub fn terminate(tid: TID)
{
    send_signal(tid, Sig::TERM, 0);
}

/// Spawns a new task, returning a [`JoinHandle`] for it.
///
/// The provided closure will be run in a new task. The join handle can be
/// used to wait for the task to complete and retrieve its return value.
///
/// # Examples
///
/// ```rust,ignore
/// let handle = std::task::spawn(|| {
///     println!("I'm running in a new task!");
///     42
/// });
///
/// let result = handle.join().unwrap();
/// assert_eq!(result, 42);
/// ```
#[status(stable)]
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    Builder::new()
    .   spawn(f)
    .   expect("Failed to spawn task")
}

/// A handle to a spawned task.
///
/// The handle can be used to wait for the task to finish and retrieve its
/// return value. If the task panics, [`join`] will return an error.
///
/// [`join`]: JoinHandle::join
#[status(stable)]
pub struct JoinHandle<T>
{
    task_id: Arc<AtomicU64>,
    result: Arc<SpinLock<Option<Result<T, TaskError>>>>,
    finished: Arc<AtomicBool>,
}

/// Error type for task failures (e.g., panic).
#[status(stable)]
#[derive(Debug, Clone, Copy)]
pub struct TaskError;

impl<T> JoinHandle<T>
{
    /// Returns the kernel-assigned task ID of the associated task.
    ///
    /// This returns `0` if the task hasn't started yet.
    #[status(stable)]
    pub fn task_id(&self) -> TID
    {
        self.task_id.load(Ordering::SeqCst)
    }

    /// Checks if the associated task has finished running.
    ///
    /// This is a non-blocking check that returns `true` if the task has
    /// completed and its result is available.
    pub fn is_finished(&self) -> bool
    {
        self.finished.load(Ordering::SeqCst)
    }

    /// Sends a signal to the associated task.
    ///
    /// This is a convenience method equivalent to `send_signal(self.task_id(), sig, data)`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::task::Sig;
    ///
    /// let handle = std::task::spawn(|| {
    ///     // Long-running task
    /// });
    ///
    /// // Send a termination signal through the handle
    /// handle.send_signal(Sig::TERM, 0);
    /// ```
    #[status(stable)]
    pub fn send_signal(&self, sig: Sig, data: u64)
    {
        let tid = self.task_id.load(Ordering::SeqCst);

        if tid != 0
        {
            ExecInterruptTask(tid, sig, data);
        }
    }

    /// Sends a termination signal to the associated task.
    ///
    /// This is a convenience method that sends [`Sig::TERM`] to request
    /// graceful termination.
    ///
    /// For immediate termination, use [`kill`].
    ///
    /// [`Sig::TERM`]: enum.Sig.html#variant.TERM
    /// [`kill`]: JoinHandle::kill
    #[status(stable)]
    pub fn terminate(&self)
    {
        self.send_signal(Sig::TERM, 0);
    }

    /// Sends an interrupt signal to the associated task.
    ///
    /// This is a convenience method that sends [`Sig::INTR`] to the task.
    ///
    /// [`Sig::INTR`]: enum.Sig.html#variant.INTR
    #[status(stable)]
    pub fn interrupt(&self)
    {
        self.send_signal(Sig::INTR, 0);
    }

    /// Forcibly terminates the associated task.
    ///
    /// This method immediately kills the task identified by this handle,
    /// without giving it a chance to run any destructors or clean up resources.
    /// After calling this method, subsequent calls to [`join`] will return
    /// `Err(TaskError)`.
    ///
    /// For graceful termination that invokes signal handlers, use [`terminate`].
    ///
    /// # Safety note
    ///
    /// Use this with caution. Killing a task can leave shared resources in an
    /// inconsistent state if the task was holding locks or performing I/O
    /// operations.
    ///
    /// [`join`]: JoinHandle::join
    /// [`terminate`]: JoinHandle::terminate
    #[status(stable)]
    pub fn kill(&self)
    {
        let tid = self.task_id.load(Ordering::SeqCst);

        if tid != 0
        {
            ExecKillTaskById(tid);
        }
    }

    /// Waits for the associated task to finish.
    ///
    /// This function will block the current task until the spawned task
    /// completes. If the task completed successfully, returns `Ok(T)` with
    /// the return value. If the task panicked, returns `Err(TaskError)`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let handle = std::task::spawn(|| 42);
    /// match handle.join() {
    ///     Ok(value) => println!("Task returned: {}", value),
    ///     Err(_) => println!("Task panicked!"),
    /// }
    /// ```
    pub fn join(self) -> Result<T, TaskError>
    {
        // Spin until task_id is set by the child task
        loop
        {
            let tid = self.task_id.load(Ordering::SeqCst);

            if tid != 0
            {
                break;
            }

            core::hint::spin_loop();
        }

        // Wait for the task to finish using kernel wait
        let tid = self.task_id.load(Ordering::SeqCst);
        let _exit_code = ExecWait(tid);

        // Retrieve the result
        let mut guard = self.result.lock();
        guard.take().ok_or(TaskError)?
    }
}

/// Task builder with configuration options.
///
/// Use this to spawn tasks with custom names, priorities, or CPU affinity.
///
/// # Examples
///
/// ```rust,ignore
/// let handle = std::task::Builder::new()
///     .name("worker-task".to_string())
///     .priority(5)
///     .spawn(|| {
///         println!("Worker task running!");
///     })
///     .unwrap();
/// ```
#[status(stable)]
pub struct Builder
{
    name: Option<String>,
    priority: Option<Priority>,
    cpu_affinity: Option<usize>,
}

impl Builder
{
    /// Creates a new builder with default settings.
    pub fn new() -> Builder
    {
        Builder
        {
            name: None,
            priority: None,
            cpu_affinity: None,
        }
    }

    /// Sets the name of the task.
    ///
    /// The name is used for debugging and monitoring purposes.
    pub fn name<S: Into<String>>(mut self, name: S) -> Builder
    {
        self.name = Some(name.into());
        self
    }

    /// Sets the priority of the task.
    ///
    /// Higher values typically indicate higher priority. The exact semantics
    /// depend on the kernel scheduler.
    pub fn priority(mut self, p: Priority) -> Builder
    {
        self.priority = Some(p);
        self
    }

    /// Sets the CPU affinity of the task.
    ///
    /// The task will only run on the specified CPU core. This can be useful
    /// for performance-critical tasks or for isolating tasks to specific cores.
    pub fn affinity(mut self, cpu: usize) -> Builder
    {
        self.cpu_affinity = Some(cpu);
        self
    }

    /// Spawns the task with the configured settings.
    ///
    /// Returns a [`JoinHandle`] that can be used to wait for the task to complete.
    pub fn spawn<F, T>(self, f: F) -> Result<JoinHandle<T>, TaskError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let task_id = Arc::new(AtomicU64::new(0));
        let result = Arc::new(SpinLock::new(None));
        let finished = Arc::new(AtomicBool::new(false));

        let task_id_clone = task_id.clone();
        let result_clone = result.clone();
        let finished_clone = finished.clone();

        let wrapper = move ||
        {
            // Record our task ID so the parent can wait on us
            let tid = ExecTaskId();
            task_id_clone.store(tid, Ordering::SeqCst);

            // Execute the user closure and capture the result
            // TODO: Handle panics properly by catching them and storing Err
            let value = f();
            *result_clone.lock() = Some(Ok(value));
            finished_clone.store(true, Ordering::SeqCst);
        };

        let name = self.name.unwrap_or_else(|| String::from("spawned"));
        let priority = self.priority.unwrap_or(0);

        ExecSpawnClosure(wrapper, priority, name, self.cpu_affinity);

        Ok(JoinHandle
        {
            task_id,
            result,
            finished,
        })
    }
}

impl Default for Builder
{
    fn default() -> Self
    {
        Self::new()
    }
}

/// Forcibly terminates a task by its ID.
///
/// This function immediately kills the task identified by `tid`, without
/// giving it a chance to run any destructors or clean up resources.
///
/// # Difference from `terminate`
///
/// - `kill(tid)` terminates a task immediately without invoking signal handlers
/// - [`terminate(tid)`] sends a `SIGTERM` signal, allowing the task to perform cleanup
///
/// # Safety note
///
/// Use this with caution. Killing a task can leave shared resources in an
/// inconsistent state if the task was holding locks or performing I/O
/// operations.
///
/// [`terminate(tid)`]: fn.terminate.html
#[status(stable)]
pub fn kill(tid: TID)
{
    ExecKillTaskById(tid);
}

/// Forcibly terminates a process by its ID.
///
/// This function immediately kills the process identified by `pid` and all
/// of its tasks, without giving them a chance to run any destructors or
/// clean up resources.
///
/// # Difference from `kill`
///
/// - [`kill(tid)`] terminates a single task within a process.
/// - `kill_proc(pid)` terminates the entire process and all its tasks.
///
/// # Safety note
///
/// Use this with caution. Killing a process can leave shared resources
/// (files, locks, memory mappings) in an inconsistent state, as none of
/// the tasks within the process will have a chance to clean up.
///
/// # Examples
///
/// ```rust,ignore
/// // Terminate a specific process by its PID
/// std::task::kill_proc(42);
/// ```
///
/// [`kill(tid)`]: fn.kill.html
#[status(stable)]
pub fn kill_proc(pid: PID)
{
    ExecKillProcById(pid);
}

pub type SpinLock<T> = ketypes::Nutex<T>;
