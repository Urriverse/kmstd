use crate::ExecPwd;

pub fn current_dir() -> super::fs::Result<super::fs::Path> {
    match ExecPwd() {
        Some(p) => Ok(p),
        None => Err(core::io::Error::new(core::io::ErrorKind::InvalidData, "Unknown task"))
    }
}
