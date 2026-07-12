/// 64-bit UNIX timestamp.
#[status(stable)]
pub type Time = u64;

/// Representation of the various file timestamps.
#[status(unstable)]
pub struct FileTimes
{
    accessed: Option<Time>,
    modified: Option<Time>,
}

#[status(unstable)]
impl FileTimes
{
    /// Create a new `FileTimes` with no time values set.
    pub fn new() -> Self
    {
        FileTimes
        {
            accessed: None,
            modified: None
        }
    }
    
    /// Set the last access time of a file.
    pub fn set_accessed(&mut self, t: Time) -> &mut Self
    {
        self.accessed = Some(t);
        self
    }
    
    /// Set the last modification time of a file.
    pub fn set_modified(&mut self, t: Time) -> &mut Self
    {
        self.modified = Some(t);
        self
    }
}

#[status(unstable)]
impl Default for FileTimes
{
    fn default() -> Self
    {
        Self::new()
    }
}
