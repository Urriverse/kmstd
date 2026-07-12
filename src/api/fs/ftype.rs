use std::raw::Kind;

#[status(stable)]
pub struct FileType
(
    pub(crate) Kind
);

impl FileType
{
    /// Tests whether this file type represents a directory.
    #[status(stable)]
    pub fn is_dir(&self) -> bool
    {
        self.0 == Kind::Directory
    }

    /// Tests whether this file type represents a regular file.
    #[status(stable)]
    pub fn is_file(&self) -> bool
    {
        self.0 == Kind::File
    }

    /// Tests whether this file type represents a symbolic link.
    #[status(stable)]
    pub fn is_symlink(&self) -> bool
    {
        self.0 == Kind::SymLink
    }
}
