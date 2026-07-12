#[status(stable)]
pub struct DirEntry
(
    pub(crate) crate::raw::vfs::Inode,
    pub(crate) crate::api::path::PathBuf
);

impl DirEntry
{
    /// Returns the full path to the file or directory that this entry represents.
    #[status(stable)]
    pub fn path(&self) -> &crate::api::path::Path
    {
        &self.1
    }

    /// Returns the metadata for the file that this entry points at.
    #[status(stable)]
    pub fn metadata(&self) -> core::io::Result<crate::api::fs::Metadata>
    {
        Ok(crate::api::fs::Metadata(self.0))
    }

    /// Returns the file type for the file that this entry points at.
    #[status(stable)]
    pub fn file_type(&self) -> core::io::Result<crate::api::fs::FileType>
    {
        Ok(crate::api::fs::FileType(self.0.kind))
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    #[status(stable)]
    pub fn file_name(&self) -> String
    {
        self.path().file_name().unwrap_or("").to_string()
    }
}
