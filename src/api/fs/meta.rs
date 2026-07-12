use std::fs::{ftype::FileType, perm::Permissions, time::Time};
use std::raw::{Inode, Kind, InodeFlags};

#[status(stable)]
pub struct Metadata
(
    pub(crate) Inode
);

impl Metadata
{
    /// Returns the file type for this metadata.
    #[status(stable)]
    pub fn file_type(&self) -> FileType
    {
        FileType(self.0.kind)
    }

    /// Returns `true` if this metadata is for a directory.
    #[status(stable)]
    pub fn is_dir(&self) -> bool
    {
        self.0.kind == Kind::Directory
    }

    /// Returns `true` if this metadata is for a regular file.
    #[status(stable)]
    pub fn is_file(&self) -> bool
    {
        self.0.kind == Kind::File
    }

    /// Returns `true` if this metadata is for a symbolic link.
    #[status(stable)]
    pub fn is_symlink(&self) -> bool
    {
        self.0.kind == Kind::SymLink
    }

    /// Returns the size of the file, in bytes, this metadata is for.
    #[status(stable)]
    pub fn len(&self) -> u64
    {
        self.0.size
    }

    /// Returns the permissions of the file this metadata is for.
    #[status(stable)]
    pub fn permissions(&self) -> Permissions
    {
        Permissions::from_readonly
        (
            self.0.flags &
            (   InodeFlags::USER_READ
            |   InodeFlags::GROUP_READ
            |   InodeFlags::OTHER_READ
            )   !=  0
        )
    }
    
    /// Returns the last access time.
    #[status(stable)]
    pub fn accessed(&self) -> Time
    {
        self.0.atime
    }
    
    /// Returns the last modification time.
    #[status(stable)]
    pub fn modified(&self) -> Time
    {
        self.0.mtime
    }
    
    /// Returns the creation time.
    #[status(stable)]
    pub fn created(&self) -> Time
    {
        self.0.ctime
    }
}
