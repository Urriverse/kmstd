#![feature(prelude_import)]
#![no_std]
#![feature(decl_macro)]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
#[macro_use]
extern crate ketypes;
pub mod log {
    pub macro trace {
        ($($arg : tt)+) => { crate ::MonLog(ketypes::mon::lvl::KeAttLvl::Trace,
        concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(),
        format_args!($($arg)+)); }
    }
    pub macro debug {
        ($($arg : tt)+) => { crate ::MonLog(ketypes::mon::lvl::KeAttLvl::Debug,
        concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(),
        format_args!($($arg)+)); }
    }
    pub macro info {
        ($($arg : tt)+) => { crate ::MonLog(ketypes::mon::lvl::KeAttLvl::Info,
        concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(),
        format_args!($($arg)+)); }
    }
    pub macro warn {
        ($($arg : tt)+) => { crate ::MonLog(ketypes::mon::lvl::KeAttLvl::Warn,
        concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(),
        format_args!($($arg)+)); }
    }
    pub macro error {
        ($($arg : tt)+) => { crate ::MonLog(ketypes::mon::lvl::KeAttLvl::Error,
        concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(),
        format_args!($($arg)+)); }
    }
    pub macro panic_msg {
        ($($arg : tt)+) => { crate ::MonLog(ketypes::mon::lvl::KeAttLvl::Panic,
        concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(),
        format_args!($($arg)+)); }
    }
}
pub mod util {
    pub macro SYMBOL {
        ($($v : vis $n : ident : $t : ty = $d : expr;)+) => { $(#[used] #[unsafe
        (no_mangle)] $v static $n : $t = $d;)* }
    }
    pub macro entry {
        (mod $n : literal; $($b : tt)*) => { #[cfg(not(test))] panic_handler![]; SYMBOL!
        { pub MODNAME : &'static str = $n; } #[unsafe (no_mangle)] pub extern "C" fn
        _start() { $($b)* } }
    }
}
pub mod sym {
    macro x {
        ($($y : ident)+) => { $(pub mod $y; #[allow(unused)] pub use $y ::*;)+ }
    }
    pub mod device {
        use ketypes::*;
    }
    #[allow(unused)]
    pub use device::*;
    pub mod event {}
    #[allow(unused)]
    pub use event::*;
    pub mod exec {
        use ketypes::*;
        use crate::*;
        #[allow(non_snake_case)]
        fn __stub_ExecYield() {
            {
                crate::MonLog(
                    ketypes::mon::lvl::KeAttLvl::Error,
                    "nk::nk::sym::exec",
                    "src/sym/exec.rs",
                    19u32,
                    format_args!("ExecYield not provided"),
                );
            }
        }
        #[used]
        #[allow(non_upper_case_globals)]
        #[unsafe(export_name = "KiExecYield")]
        static _ExecYield: ::ketypes::ImExport = ::ketypes::ImExport(
            __stub_ExecYield as *const (),
            ::ketypes::parse_version("0.0"),
        );
        #[allow(non_snake_case)]
        #[inline(always)]
        pub fn ExecYield() {
            (unsafe { core::mem::transmute::<_, fn()>(_ExecYield.0) })()
        }
    }
    #[allow(unused)]
    pub use exec::*;
    pub mod fs {}
    #[allow(unused)]
    pub use fs::*;
    pub mod mem {
        use crate::*;
        use ketypes::*;
    }
    #[allow(unused)]
    pub use mem::*;
    pub mod module {}
    #[allow(unused)]
    pub use module::*;
    pub mod mon {
        use ketypes::*;
    }
    #[allow(unused)]
    pub use mon::*;
    pub mod paging {}
    #[allow(unused)]
    pub use paging::*;
}
pub mod macros {
    pub macro panic_handler {
        () => { #[panic_handler] fn ___km_ph(x : & core::panic::PanicInfo) -> ! { crate
        ::ExecPanic(x) } }
    }
    pub macro exit {
        () => { crate ::ExecExit(0) }, ($code : expr) => { crate ::ExecExit($code) },
    }
}
pub mod ga {
    use crate::sym::*;
    struct __GA;
    #[used]
    static __GA_INSTANCE: __GA = __GA;
    const _: () = {
        #[rustc_std_internal_symbol]
        #[rustc_allocator]
        unsafe fn __rust_alloc(size: usize, align: ::core::mem::Alignment) -> *mut u8 {
            ::core::alloc::GlobalAlloc::alloc(
                &__GA_INSTANCE,
                ::core::alloc::Layout::from_size_alignment_unchecked(size, align),
            )
        }
        #[rustc_std_internal_symbol]
        #[rustc_deallocator]
        unsafe fn __rust_dealloc(
            ptr: *mut u8,
            size: usize,
            align: ::core::mem::Alignment,
        ) -> () {
            ::core::alloc::GlobalAlloc::dealloc(
                &__GA_INSTANCE,
                ptr,
                ::core::alloc::Layout::from_size_alignment_unchecked(size, align),
            )
        }
        #[rustc_std_internal_symbol]
        #[rustc_reallocator]
        unsafe fn __rust_realloc(
            ptr: *mut u8,
            size: usize,
            align: ::core::mem::Alignment,
            new_size: usize,
        ) -> *mut u8 {
            ::core::alloc::GlobalAlloc::realloc(
                &__GA_INSTANCE,
                ptr,
                ::core::alloc::Layout::from_size_alignment_unchecked(size, align),
                new_size,
            )
        }
        #[rustc_std_internal_symbol]
        #[rustc_allocator_zeroed]
        unsafe fn __rust_alloc_zeroed(
            size: usize,
            align: ::core::mem::Alignment,
        ) -> *mut u8 {
            ::core::alloc::GlobalAlloc::alloc_zeroed(
                &__GA_INSTANCE,
                ::core::alloc::Layout::from_size_alignment_unchecked(size, align),
            )
        }
    };
    unsafe impl core::alloc::GlobalAlloc for __GA {
        unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
            MemAlloc(layout)
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
            MemFree(ptr, layout)
        }
    }
}
pub mod fs {}
pub mod front {
    mod device {
        use alloc::boxed::Box;
        #[repr(C, align(128))]
        pub struct Device;
        impl Device {
            #[inline(always)]
            pub fn new(name: &str) -> Option<Box<Self>> {
                VtDeviceNew(name)
            }
        }
    }
    pub use device::*;
    pub mod arch {
        mod amd64 {
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = "KiArchTimeFromBootNs")]
            static _UptimeNs: ::ketypes::ImExport = ::ketypes::ImExport(
                0 as *const (),
                ::ketypes::parse_version("0.1"),
            );
            /// # UptimeNs
            ///
            /// Returns current uptime in milliseconds
            #[allow(non_snake_case)]
            #[inline(always)]
            fn UptimeNs() -> u64 {
                (unsafe { core::mem::transmute::<_, fn() -> u64>(_UptimeNs.0) })()
            }
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = "KiArchTimeFromBoot")]
            static _Uptime: ::ketypes::ImExport = ::ketypes::ImExport(
                0 as *const (),
                ::ketypes::parse_version("0.1"),
            );
            /// # Uptime
            ///
            /// Returns current uptime in seconds
            #[allow(non_snake_case)]
            #[inline(always)]
            fn Uptime() -> f64 {
                (unsafe { core::mem::transmute::<_, fn() -> f64>(_Uptime.0) })()
            }
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = "KiGtArchTotalCpus")]
            static _TotalCpus: ::ketypes::ImExport = ::ketypes::ImExport(
                0 as *const (),
                ::ketypes::parse_version("0.1"),
            );
            /// # TotalCpus
            ///
            /// Returns amount of CPUs detected by kernel/bootloader.
            #[allow(non_snake_case)]
            #[inline(always)]
            pub fn TotalCpus() -> usize {
                (unsafe { core::mem::transmute::<_, fn() -> usize>(_TotalCpus.0) })()
            }
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = "KiArchEndOfInterrupt")]
            static _EndOfInterrupt: ::ketypes::ImExport = ::ketypes::ImExport(
                0 as *const (),
                ::ketypes::parse_version("0.1"),
            );
            /// # EndOfInterrupt
            ///
            /// Sends End Of Interrupt to Local APIC.
            #[allow(non_snake_case)]
            #[inline(always)]
            pub fn EndOfInterrupt() {
                (unsafe { core::mem::transmute::<_, fn()>(_EndOfInterrupt.0) })()
            }
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = "KiArchRdpidAvailable")]
            static _ArchRdpidAvailable: ::ketypes::ImExport = ::ketypes::ImExport(
                0 as *const (),
                ::ketypes::parse_version("0.1"),
            );
            /// # ArchRdpidAvailable
            ///
            /// Checks if RDPID available. Used in [`currect_cpu`]. Intentionally private.
            #[doc(hidden)]
            #[allow(non_snake_case)]
            #[inline(always)]
            fn ArchRdpidAvailable() -> bool {
                (unsafe {
                    core::mem::transmute::<_, fn() -> bool>(_ArchRdpidAvailable.0)
                })()
            }
            /// # rdpid_raw
            ///
            /// Returns RDPID value. Intentionally private.
            #[doc(hidden)]
            #[inline(always)]
            fn rdpid_raw() -> usize {
                let id: u64;
                unsafe {
                    asm!("rdpid {0}", out(reg) id, options(preserves_flags, nostack))
                }
                id as usize
            }
            /// # rdmsr
            ///
            /// Reads Machine-Specific Register and returns its value.
            /// Intentionally private.
            #[inline]
            unsafe fn rdmsr(msr: u32) -> u64 {
                let (lo, hi): (u32, u32);
                unsafe {
                    asm!(
                        "rdmsr", in ("ecx") msr, out("eax") lo, out("edx") hi,
                        options(preserves_flags, nostack)
                    );
                }
                ((hi as u64) << 32) | (lo as u64)
            }
            /// # CurrentCpu
            ///
            /// Returns current CPU unique identifier.
            #[inline(always)]
            #[allow(non_snake_case)]
            pub fn CurrentCpu() -> usize {
                if ArchRdpidAvailable() {
                    rdpid_raw()
                } else {
                    unsafe { rdmsr(3221225731) as usize }
                }
            }
            /// Delivery modes for IPIs.
            ///
            /// These are the bits that are OR‑ed into the ICR (Interrupt Command Register)
            /// to specify the delivery semantics of the IPI.
            #[repr(u32)]
            pub enum DeliveryMode {
                /// Deliver the interrupt to the target processor(s).
                Fixed = 0b000 << 8,
                /// Deliver to the processor with the lowest priority.
                LowestPri = 0b001 << 8,
                /// System Management Interrupt.
                Smi = 0b010 << 8,
                /// Non‑Maskable Interrupt.
                Nmi = 0b100 << 8,
                /// INIT IPI (reset the target processor).
                Init = 0b101 << 8,
                /// Startup IPI (used for AP boot).
                StartUp = 0b110 << 8,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for DeliveryMode {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            DeliveryMode::Fixed => "Fixed",
                            DeliveryMode::LowestPri => "LowestPri",
                            DeliveryMode::Smi => "Smi",
                            DeliveryMode::Nmi => "Nmi",
                            DeliveryMode::Init => "Init",
                            DeliveryMode::StartUp => "StartUp",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            unsafe impl ::core::clone::TrivialClone for DeliveryMode {}
            #[automatically_derived]
            impl ::core::clone::Clone for DeliveryMode {
                #[inline]
                fn clone(&self) -> DeliveryMode {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for DeliveryMode {}
            /// A complete snapshot of the CPU state at the time of an interrupt, exception, or system call.
            #[repr(C)]
            pub struct TrapFrame {
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
            #[automatically_derived]
            impl ::core::fmt::Debug for TrapFrame {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "rax",
                        "rbx",
                        "rcx",
                        "rdx",
                        "rsi",
                        "rdi",
                        "rbp",
                        "r8",
                        "r9",
                        "r10",
                        "r11",
                        "r12",
                        "r13",
                        "r14",
                        "r15",
                        "rip",
                        "cs",
                        "rflags",
                        "rsp",
                        "ss",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.rax,
                        &self.rbx,
                        &self.rcx,
                        &self.rdx,
                        &self.rsi,
                        &self.rdi,
                        &self.rbp,
                        &self.r8,
                        &self.r9,
                        &self.r10,
                        &self.r11,
                        &self.r12,
                        &self.r13,
                        &self.r14,
                        &self.r15,
                        &self.rip,
                        &self.cs,
                        &self.rflags,
                        &self.rsp,
                        &&self.ss,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "TrapFrame",
                        names,
                        values,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            unsafe impl ::core::clone::TrivialClone for TrapFrame {}
            #[automatically_derived]
            impl ::core::clone::Clone for TrapFrame {
                #[inline]
                fn clone(&self) -> TrapFrame {
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for TrapFrame {}
        }
        pub use amd64::*;
    }
}
pub use macros::*;
pub use util::*;
pub use log::*;
pub use sym::*;
pub use front::*;
