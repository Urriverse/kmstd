pub enum Format {
    Pretty,
    Regular,
}

pub trait Sink: core::fmt::Write + Sync + Send {
    fn format(&self) -> Format;
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum AttLvl {
    /// Unrecoverable error – system will panic or halt.
    Panic = 0,
    /// Recoverable error.
    Error = 1,
    /// Warning – unexpected but non‑fatal.
    Warn = 2,
    /// Informational message.
    Info = 3,
    /// Debugging information (disabled by `lowlog`).
    Debug = 4,
    /// Trace level (very verbose, disabled by `lowlog`).
    Trace = 5,
}

impl AttLvl {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Panic => "PANIC",
            Self::Error => "ERROR",
            Self::Warn  => " WARN",
            Self::Info  => " INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }

    pub fn pretty(self) -> &'static str {
        match self {
            Self::Panic => "\x1b[35;1mPANIC\x1b[0m",
            Self::Error => "\x1b[31;1mERROR\x1b[0m",
            Self::Warn  => "\x1b[33;1m WARN\x1b[0m",
            Self::Info  => "\x1b[32;1m INFO\x1b[0m",
            Self::Debug => "\x1b[36;1mDEBUG\x1b[0m",
            Self::Trace => "\x1b[90;1mTRACE\x1b[0m",
        }
    }
}

Import! {
    pub fn MonLog(level: AttLvl, module: &'static str, file: &'static str, line: u32, args: core::fmt::Arguments<'_>) where kernel 0.1;
    pub fn MonAddSink(sink: &'static mut dyn Sink) where kernel 0.1;
}
