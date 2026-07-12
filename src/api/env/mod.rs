//! Inspection of the task's environment.

#[status(experimental)]
pub fn current_dir() -> super::fs::Result<super::fs::Path>
{
    match crate::raw::ExecPwd()
    {
        Some(p) => Ok(p),
        None => Err(core::io::Error::new(core::io::ErrorKind::InvalidData, "Unknown task"))
    }
}
