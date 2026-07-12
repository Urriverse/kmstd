//! AMD64 (x86_64) specific kernel interfaces and CPU primitives.
//!
//! This module defines the CPU state representation ([`TrapFrame`]), interrupt
//! delivery modes, page table entry flags, and low-level I/O port and MSR
//! accessors specific to the x86_64 architecture.

/// A complete snapshot of the CPU state at the time of an interrupt, exception, or system call.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame
{
    /// RAX – general‑purpose register, often used as the syscall number and return value.
    pub rax: u64,
    /// RBX – general‑purpose register, saved but not used for syscalls.
    pub rbx: u64,
    /// RCX – general‑purpose register; on `syscall` entry, it holds the user return address (`RIP`).
    pub rcx: u64,
    /// RDX – general‑purpose register, often used as the third syscall argument.
    pub rdx: u64,
    /// RSI – general‑purpose register, used as the second syscall argument.
    pub rsi: u64,
    /// RDI – general‑purpose register, used as the first syscall argument.
    pub rdi: u64,
    /// RBP – base pointer, saved for stack frame debugging.
    pub rbp: u64,
    /// R8 – general‑purpose register, used as the fifth syscall argument.
    pub r8: u64,
    /// R9 – general‑purpose register, used as the sixth syscall argument.
    pub r9: u64,
    /// R10 – general‑purpose register, used as the fourth syscall argument
    /// (the `syscall` instruction clobbers RCX and R11, so R10 is used instead
    /// of RCX for the fourth argument).
    pub r10: u64,
    /// R11 – general‑purpose register; on `syscall` entry, it holds the user `RFLAGS`.
    pub r11: u64,
    /// R12 – general‑purpose register, callee‑saved.
    pub r12: u64,
    /// R13 – general‑purpose register, callee‑saved.
    pub r13: u64,
    /// R14 – general‑purpose register, callee‑saved.
    pub r14: u64,
    /// R15 – general‑purpose register, callee‑saved.
    pub r15: u64,
    /// Instruction pointer – the address to return to after the interrupt.
    pub rip: u64,
    /// Code segment selector (with RPL) – indicates the privilege level of
    /// the interrupted context (e.g., `0x08 | 0` for kernel, `0x18 | 3` for user).
    pub cs: u64,
    /// RFLAGS register – contains CPU flags (interrupt flag, direction flag, etc.).
    pub rflags: u64,
    /// Stack pointer – the user or kernel stack pointer at the time of the
    /// interrupt.
    pub rsp: u64,
    /// Stack segment selector – used with `RSP` to form the full stack address.
    pub ss: u64,
}

/// Delivery modes for IPIs.
///
/// These are the bits that are OR‑ed into the ICR (Interrupt Command Register)
/// to specify the delivery semantics of the IPI.
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum DeliveryMode
{
    /// Deliver the interrupt to the target processor(s).
    Fixed        = 0b000 << 8,
    /// Deliver to the processor with the lowest priority.
    LowestPri    = 0b001 << 8,
    /// System Management Interrupt.
    Smi          = 0b010 << 8,
    /// Non‑Maskable Interrupt.
    Nmi          = 0b100 << 8,
    /// INIT IPI (reset the target processor).
    Init         = 0b101 << 8,
    /// Startup IPI (used for AP boot).
    StartUp      = 0b110 << 8,
}

bitflags::bitflags!
{
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct EntryFlags: u64
    {
        /// The page is present in memory.
        const PRESENT         = 1 <<  0;
        /// The page is writable (for kernel mode, or user if `USER_ACCESSIBLE`).
        const WRITABLE        = 1 <<  1;
        /// The page is accessible from user mode (CPL 3).
        const USER_ACCESSIBLE = 1 <<  2;
        /// Write‑through caching (vs. write‑back).
        const WRITE_THROUGH   = 1 <<  3;
        /// Cache disabled for this page.
        const CACHE_DISABLE   = 1 <<  4;
        /// The page has been accessed (set by hardware).
        const ACCESSED        = 1 <<  5;
        /// The page has been written to (set by hardware).
        const DIRTY           = 1 <<  6;
        /// The entry points to a huge page (2 MiB or 1 GiB).
        const HUGE_PAGE       = 1 <<  7;
        /// The page is global (not flushed on CR3 switch).
        const GLOBAL          = 1 <<  8;
        /// Execute disable (NX bit) – the page cannot be executed.
        const NO_EXECUTE      = 1 << 63;

        // Kernel‑specific software‑managed flags (stored in available bits).
        /// Copy‑on‑write flag (used by the scheduler).
        const COPY_ON_WRITE   = 1 << 52;
        /// File‑mapped flag (for mmap).
        const FILE_MAPPED     = 1 << 53;
        /// Swapped flag (page is swapped out).
        const SWAPPED         = 1 << 54;
    }
}

/// This type represents interrupt vector. use first field as a vector (`v.0`).
/// 
/// Note: don't drop [`Vector`] 'til you set your ISR, otherwise other module
/// can grab your ISR instead of you. Drop Vector only when your ISR already set.
/// 
/// Note: you MUST drop [`Vector`] after use as all tasks which wanna allocate
/// new vector are waiting 'til you hold the [`Vector`].
#[repr(C)]
pub struct Vector
(
    pub u8, usize
);

bitflags::bitflags!
{
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GsiRouteBits: u32
    {
        const ACTIVE_LOW = 1 << 13;
        const LEVEL_TRIG = 1 << 15;
    }
}

impl Drop for Vector
{
    fn drop(&mut self)
    {
        ReleaseVector(self);
    }
}

#[doc(hidden)]
#[inline(always)]
fn rdpid_raw() -> usize
{
    let id: u64;

    unsafe
    {
        core::arch::asm!
        {
            "rdpid {}",
            out(reg) id,
            options(nostack, preserves_flags),
        }
    }

    id as usize
}

#[inline(always)]
fn rdmsr(msr: u32) -> u64
{
    let (lo, hi): (u32, u32);

    unsafe
    {
        core::arch::asm!
        {
            "rdmsr",
            in("ecx") msr,
            out("eax") lo,
            out("edx") hi,
            options(nostack, preserves_flags),
        }
    }

    ((hi as u64) << 32) | (lo as u64)
}

#[inline(always)]
fn in8(port: u16) -> u8
{
    unsafe
    {
        let rv: u8;
        
        core::arch::asm!
        {
            "in {}, {:x}",
            out(reg_byte) rv,
            in(reg) port
        };
        
        rv
    }
}

#[inline(always)]
fn in16(port: u16) -> u16
{
    unsafe
    {
        let rv: u16;
        
        core::arch::asm!
        {
            "in {:x}, {:x}",
            out(reg) rv,
            in(reg) port
        };
        
        rv
    }
}

#[inline(always)]
fn in32(port: u16) -> u32
{
    unsafe
    {
        let rv: u32;
        
        core::arch::asm!
        {
            "in {:l}, {:x}",
            out(reg) rv,
            in(reg) port
        };
        
        rv
    }
}

#[inline(always)]
fn out8(port: u16, v: u8)
{
    unsafe
    {
        core::arch::asm!
        {
            "in {:x}, {}",
            in(reg) port,
            in(reg_byte) v
        }
    }
}

#[inline(always)]
fn out16(port: u16, v: u16)
{
    unsafe
    {
        core::arch::asm!
        {
            "in {:x}, {:x}",
            in(reg) port,
            in(reg) v
        }
    }
}

#[inline(always)]
fn out32(port: u16, v: u32)
{
    unsafe
    {
        core::arch::asm!
        {
            "in {:x}, {:l}",
            in(reg) port,
            in(reg) v
        }
    }
}

// Reads value from I/O port. `T` must be of size 1, 2, or 4 bytes.
#[inline(always)]#[allow(non_snake_case)]
pub unsafe fn PortRead<T: From<usize>>(port: u16) -> T
{
    match size_of::<T>()
    {
        1 =>  in8(port) as usize,
        2 => in16(port) as usize,
        4 => in32(port) as usize,
        _ => panic!("Invalid type"),
    }
    .   into()
}

// Writes value to I/O port. `T` must be of size 1, 2, or 4 bytes.
#[inline(always)]#[allow(non_snake_case)]
pub unsafe fn PortWrite<T: Into<usize>>(port: u16, v: T)
{
    match size_of::<T>()
    {
        1 =>  out8(port, v.into() as  u8),
        2 => out16(port, v.into() as u16),
        4 => out32(port, v.into() as u32),
        _ => panic!("Invalid type"),
    }
}

/// Returns current CPU unique identifier.
#[inline(always)]#[allow(non_snake_case)]
pub fn CurrentCpu() -> usize
{
    if ArchRdpidAvailable()
    {
        rdpid_raw()
    }
    else
    {
        rdmsr(3221225731) as usize
    }
}

Import!
{
    /// Replaces current system call handler to your one.
    /// Returns [`None`] if current task isn't part of process.
    pub fn ArchReplaceSyscallHandler as OnSystemCall(sh: fn(&mut TrapFrame)) -> Option<()>
    where kernel 0.1;

    /// Returns current uptime in milliseconds
    pub fn ArchTimeFromBootNs as UptimeNs() -> u64
    where kernel 0.1;

    /// Returns current uptime in seconds
    pub fn ArchTimeFromBoot as Uptime() -> f64
    where kernel 0.1;

    /// Returns amount of CPUs detected by kernel/bootloader.
    pub fn GtArchTotalCpus as TotalCpus() -> usize
    where kernel 0.1;

    /// Sends End Of Interrupt to Local APIC.
    pub fn ArchEndOfInterrupt as EndOfInterrupt()
    where kernel 0.1;

    /// Sends the IPI to the target APIC ID.
    pub fn ArchSendIPI as SendIPI(target_cpu_id: u32, event_vector: u8, mode: DeliveryMode)
    where kernel 0.1;

    #[doc(hidden)]
    fn ArchRdpidAvailable as ArchRdpidAvailable() -> bool
    where kernel 0.1;

    /// Sets ISR on all CPUs.
    pub fn ArchSetInterruptServiceRoutine as SetISR(vector: u8, routine: *const ())
    where kernel 0.1;

    /// Removes ISR on all CPUs. Other modules/tasks can use this vector again.
    pub fn ArchRemoveInterruptServiceRoutine as RemoveISR(vector: u8)
    where kernel 0.1;

    /// Configures GSI route.
    pub fn ArchRouteGsi as RouteGsi(gsi: u32, cpuid: usize, mask: bool, vector: u8, bits: GsiRouteBits) -> Result<(), ()>
    where kernel 0.1;

    /// Allocates new vector for your ISR.
    pub fn ArchAllocateVector as AllocateVector() -> Option<Vector>
    where kernel 0.1;

    pub fn ArchReleaseVector as ReleaseVector(v: &mut Vector)
    where kernel 0.1;
}
