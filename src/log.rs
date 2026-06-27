#[derive(Clone, Copy)]
#[repr(u8)]
pub enum AttLvl {
    Panic,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
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

pub macro trace($($arg:tt)+) {
    crate::sym::k_log(AttLvl::Trace, concat!(env!("CARGO_PKG_NAME");, "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
}

pub macro debug($($arg:tt)+) {
    crate::sym::k_log(AttLvl::Trace, concat!(env!("CARGO_PKG_NAME");, "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
}

pub macro info($($arg:tt)+) {
    crate::sym::k_log(AttLvl::Trace, concat!(env!("CARGO_PKG_NAME");, "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
}

pub macro warn($($arg:tt)+) {
    crate::sym::k_log(AttLvl::Trace, concat!(env!("CARGO_PKG_NAME");, "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
}

pub macro error($($arg:tt)+) {
    crate::sym::k_log(AttLvl::Trace, concat!(env!("CARGO_PKG_NAME");, "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
}

pub macro panic_msg($($arg:tt)+) {
    crate::sym::k_log(AttLvl::Trace, concat!(env!("CARGO_PKG_NAME");, "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
}
