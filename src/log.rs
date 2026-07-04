pub macro trace($($arg:tt)+){crate::MonLog(crate::AttLvl::Trace,concat
!(env!("CARGO_PKG_NAME"),"::",module_path!()),file!(),line!(),format_args!($($arg)+));}
pub macro debug($($arg:tt)+){crate::MonLog(crate::AttLvl::Debug,concat
!(env!("CARGO_PKG_NAME"),"::",module_path!()),file!(),line!(),format_args!($($arg)+));}
pub macro info($($arg:tt)+){crate::MonLog(crate::AttLvl::Info,concat
!(env!("CARGO_PKG_NAME"),"::",module_path!()),file!(),line!(),format_args!($($arg)+));}
pub macro warn($($arg:tt)+){crate::MonLog(crate::AttLvl::Warn,concat
!(env!("CARGO_PKG_NAME"),"::",module_path!()),file!(),line!(),format_args!($($arg)+));}
pub macro error($($arg:tt)+){crate::MonLog(crate::AttLvl::Error,concat
!(env!("CARGO_PKG_NAME"),"::",module_path!()),file!(),line!(),format_args!($($arg)+));}
pub macro panic_msg($($arg:tt)+){crate::MonLog(crate::AttLvl::Panic,concat
!(env!("CARGO_PKG_NAME"),"::",module_path!()),file!(),line!(),format_args!($($arg)+));}
