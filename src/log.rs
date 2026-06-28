pub macro trace($($arg:tt)+) {
    crate::sym::k_log(ketypes::mon::lvl::AttLvl::Trace, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro debug($($arg:tt)+) {
    crate::sym::k_log(ketypes::mon::lvl::AttLvl::Debug, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro info($($arg:tt)+) {
    crate::sym::k_log(ketypes::mon::lvl::AttLvl::Info, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro warn($($arg:tt)+) {
    crate::sym::k_log(ketypes::mon::lvl::AttLvl::Warn, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro error($($arg:tt)+) {
    crate::sym::k_log(ketypes::mon::lvl::AttLvl::Error, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro panic_msg($($arg:tt)+) {
    crate::sym::k_log(ketypes::mon::lvl::AttLvl::Panic, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}
