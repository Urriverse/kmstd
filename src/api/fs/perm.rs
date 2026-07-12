#[status(stable)]
pub struct Permissions
{
    pub(crate) ro: bool
}

impl Permissions
{
    /// Create `Permissions` from a readonly flag.
    #[status(stable)]
    pub fn from_readonly(readonly: bool) -> Self
    {
        Permissions
        {
            ro: readonly
        }
    }

    /// Returns `true` if these permissions describe a readonly (unwritable) file.
    #[status(stable)]
    pub fn readonly(&self) -> bool
    {
        self.ro
    }

    /// Set the readonly flag the corresponding file.
    #[status(stable)]
    pub fn set_readonly(&mut self, readonly: bool)
    {
        self.ro = readonly
    }
}
