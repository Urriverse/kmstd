#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {{
        #[cfg(not(feature = "lowlog"))]
        $crate::KeInvoke!(log: 5, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {{
        #[cfg(not(feature = "lowlog"))]
        $crate::KeInvoke!(log: 4, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {{
        $crate::KeInvoke!(log: 3, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {{
        $crate::KeInvoke!(log: 2, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {{
        $crate::KeInvoke!(log: 1, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! panic_msg {
    ($($arg:tt)+) => {{
        $crate::KeInvoke!(log: 0, concat!(crate::mod_ident!(), "::", module_path!()), file!(), line!(), &format_args!($($arg)+));
    }};
}
