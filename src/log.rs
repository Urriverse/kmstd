pub macro trace($($arg:tt)+) {
    crate::KeMonLog(ketypes::mon::lvl::KeAttLvl::Trace, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro debug($($arg:tt)+) {
    crate::KeMonLog(ketypes::mon::lvl::KeAttLvl::Debug, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro info($($arg:tt)+) {
    crate::KeMonLog(ketypes::mon::lvl::KeAttLvl::Info, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro warn($($arg:tt)+) {
    crate::KeMonLog(ketypes::mon::lvl::KeAttLvl::Warn, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro error($($arg:tt)+) {
    crate::KeMonLog(ketypes::mon::lvl::KeAttLvl::Error, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}

pub macro panic_msg($($arg:tt)+) {
    crate::KeMonLog(ketypes::mon::lvl::KeAttLvl::Panic, concat!(env!("CARGO_PKG_NAME"), "::", module_path!()), file!(), line!(), format_args!($($arg)+));
}
