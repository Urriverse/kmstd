//! File system operations.
//!
//! This module provides a safe, high-level API for interacting with the file system,
//! inspired by `std::fs`. It includes functions for reading, writing, and querying
//! files and directories, as well as types representing file metadata and directory entries.

use core::io::{Error, ErrorKind};
use alloc::{format, string::ToString, sync::Arc, vec::Vec, string::String};
pub use core::io::Result;

use crate::api::path::{Path, PathBuf};
use crate::raw::*;

insmod!
{
    pub  time    pub(crate),
    pub  meta    pub(crate),
    pub  ftype   pub(crate),
    pub  perm    pub(crate),
    pub  dent    pub(crate),
    pub  rdir    pub(crate),
}

/// Returns the canonical, absolute form of the path with all intermediate
/// components normalized and symbolic links resolved.
#[status(stable)]
pub fn canonicalize<P: AsRef<Path>>(path: P) -> Result<PathBuf>
{
    path.as_ref().canonicalize()
}

/// Creates a new, empty directory at the provided path.
#[status(stable)]
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()>
{
    let path = path.as_ref();

    let parent = path.parent().ok_or_else(||
    {
        Error::new(ErrorKind::NotFound, "Attempt to create root")
    })?;

    let (id, mb) = FsResolve(parent.as_str()).map_err(|e|
    {
        Error::new(e.into(), format!("Can't resolve `{}`: {:?}", parent, e))
    })?;

    let stat = FsStat(&mb, id).ok_or_else(||
    {
        Error::new(ErrorKind::Other, format!("Can't stat `{}`", parent))
    })?;

    if stat.kind != Kind::Directory
    {
        return Err(Error::new
        (
            ErrorKind::NotADirectory,
            format!("`{}` is not a directory", parent)
        ));
    }

    FsNew(&mb, stat, Kind::Directory).map_err(|e|
    {
        Error::new(e.into(), format!("Error: {:?}", e))
    })?;

    Ok(())
}

/// Recursively create a directory and all of its parent components if they are missing.
#[status(stable)]
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()>
{
    let path = path.as_ref();

    match create_dir(path) 
    {
        Ok(_) => Ok(()),
        
        Err(e) =>
        {
            match e.kind()
            {
                ErrorKind::NotFound =>
                {
                    match path.parent()
                    {
                        Some(parent) => create_dir_all(parent),

                        None => Err(Error::new
                        (
                            ErrorKind::NotFound,
                            "Attempt to create root"
                        ))
                    }
                },

                _ => Err(e)
            }
        }
    }
}

/// Returns `Ok(true)` if the path points at an existing entity on the filesystem.
#[status(stable)]
pub fn exists<P: AsRef<Path>>(path: P) -> Result<bool>
{
    let path = path.as_ref();

    match FsResolve(path.as_str())
    {
        Ok(_) => Ok(true),

        Err(e) => match e
        {
            FsError::NoEntry => Ok(false),
            _ => Err(Error::new(e.into(), format!("Error: {:?}", e)))
        }
    }
}

/// Given a path, query the file system to get information about a file, directory, etc.
#[status(stable)]
pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata>
{
    let path = path.as_ref();

    match FsResolve(path.as_str())
    {
        Ok((id, mb)) =>
        {
            match FsStat(&mb, id)
            {
                Some(inode) => Ok(Metadata(inode)),
                None => Err(Error::new(ErrorKind::Other, format!("Can't stat `{}`", path))),
            }
        },

        Err(e) => Err(Error::new(e.into(), format!("Error: {:?}", e))),
    }
}

/// Query the metadata about a file without following symlinks.
#[status(stable)]
pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata>
{
    let path = path.as_ref();
    
    // If the path has no parent or no file name, fall back to regular metadata
    let (parent, name) = match (path.parent(), path.file_name())
    {
        (Some(p), Some(n)) => (p, n),
        _ => return metadata(path),
    };
    
    // Resolve the parent (this may follow symlinks in the parent path)
    let (parent_id, mb) = FsResolve(parent.as_str()).map_err(|e|
    {
        Error::new(e.into(), format!("Can't resolve parent `{}`: {:?}", parent, e))
    })?;
    
    // Lookup the final component without following symlinks
    let id = FsLookup(&mb, parent_id, name).ok_or_else(||
    {
        Error::new(ErrorKind::NotFound, format!("`{}` not found", path))
    })?;
    
    // Get the metadata of the symlink itself
    let inode = FsStat(&mb, id).ok_or_else(||
    {
        Error::new(ErrorKind::Other, format!("Can't stat `{}`", path))
    })?;
    
    Ok(Metadata(inode))
}

/// Returns the last modification time listed in this metadata.
#[status(stable)]
pub fn modified<P: AsRef<Path>>(path: P) -> Result<Time>
{
    Ok(metadata(path)?.modified())
}

/// Returns the last access time listed in this metadata.
#[status(stable)]
pub fn accessed<P: AsRef<Path>>(path: P) -> Result<Time>
{
    Ok(metadata(path)?.accessed())
}

/// Returns the creation time listed in this metadata.
#[status(stable)]
pub fn created<P: AsRef<Path>>(path: P) -> Result<Time>
{
    Ok(metadata(path)?.created())
}

/// Read the entire contents of a file into a bytes vector.
#[status(stable)]
pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>>
{
    let path = path.as_ref();

    match FsResolve(path.as_str())
    {
        Ok((id, mb)) =>
        {
            match FsStat(&mb, id)
            {
                Some(inode) =>
                {
                    if inode.kind != Kind::File
                    {
                        return Err(Error::new(ErrorKind::IsADirectory, format!("Can't read `{}`: Not a file", path)))
                    }

                    let mut buf = alloc::vec![0u8; inode.size as usize];

                    match FsRead(&mb, id, 0, &mut buf)
                    {
                        Ok(_) => Ok(buf),
                        Err(e) => Err(Error::new(e.into(), format!("Failed to read `{}`: {:?}", path, e)))
                    }
                },

                None => Err(Error::new(ErrorKind::Other, format!("Can't stat `{}`", path)))
            }
        },

        Err(e) => Err(Error::new(e.into(), format!("Can't resolve `{}`: {:?}", path, e)))
    }
}

/// Returns an iterator over the entries within a directory.
#[status(stable)]
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>
{
    let path = path.as_ref();

    match FsResolve(path.as_str())
    {
        Ok((id, mb)) =>
        {
            match FsStat(&mb, id)
            {
                Some(inode) =>
                {
                    if inode.kind != Kind::Directory
                    {
                        return Err(Error::new(ErrorKind::NotADirectory, format!("Can't read dir `{}`: Not a directory", path)))
                    }

                    Ok(ReadDir(mb, id, inode.size as usize, path.into()))
                },

                None => Err(Error::new(ErrorKind::Other, format!("Can't stat `{}`", path))),
            }
        },

        Err(e) => Err(Error::new(e.into(), format!("Can't resolve `{}`: {:?}", path, e))),
    }
}

/// Reads a symbolic link, returning the file that the link points to.
#[status(stable)]
pub fn read_link<P: AsRef<Path>>(path: P) -> Result<PathBuf>
{
    let path = path.as_ref();

    match FsResolve(path.as_str())
    {
        Ok((id, mb)) =>
        {
            match FsStat(&mb, id)
            {
                Some(inode) =>
                {
                    if inode.kind != Kind::SymLink
                    {
                        return Err(Error::new(ErrorKind::IsADirectory, format!("Can't read symlink `{}`: Not a symlink", path)))
                    }

                    let mut buf = alloc::vec![0u8; inode.size as usize];

                    match FsReadLink(&mb, id, 0, &mut buf)
                    {
                        Ok(_) => Ok(PathBuf::from(String::from_utf8_lossy_owned(buf).to_string())),
                        Err(e) => Err(Error::new(e.into(), format!("Failed to read `{}`: {:?}", path, e)))
                    }
                },

                None => Err(Error::new(ErrorKind::Other, format!("Can't stat `{}`", path)))
            }
        },

        Err(e) => Err(Error::new(e.into(), format!("Can't resolve `{}`: {:?}", path, e)))
    }
}

/// Read the entire contents of a file into a string.
#[status(stable)]
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String>
{
    Ok(String::from_utf8_lossy(&read(path)?).to_string())
}

/// Write a slice as the entire contents of a file.
#[status(stable)]
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()>
{
    let path = path.as_ref();

    match FsResolve(path.as_str())
    {
        Ok((id, mb)) =>
        {
            match FsStat(&mb, id)
            {
                Some(inode) =>
                {
                    if inode.kind != Kind::File
                    {
                        return Err(Error::new(ErrorKind::IsADirectory, format!("Can't write `{}`: Not a file", path)))
                    }

                    match FsWrite(&mb, id, 0, contents.as_ref())
                    {
                        Ok(_) => Ok(()),
                        Err(e) => Err(Error::new(e.into(), format!("Failed to write `{}`: {:?}", path, e)))
                    }
                },

                None => Err(Error::new(ErrorKind::Other, format!("Can't stat `{}`", path)))
            }
        },

        Err(e) => Err(Error::new(e.into(), format!("Can't resolve `{}`: {:?}", path, e)))
    }
}

/// Removes an empty directory.
///
/// If the directory is not empty, an error will be returned.
#[status(stable)]
pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<()>
{
    let path = path.as_ref();
    
    let (parent, name) = match (path.parent(), path.file_name())
    {
        (Some(p), Some(n)) => (p, n),
        _ => return Err(Error::new(ErrorKind::Other, "Cannot remove root")),
    };
    
    let (parent_id, mb) = FsResolve(parent.as_str()).map_err(|e|
    {
        Error::new(e.into(), format!("Can't resolve parent `{}`: {:?}", parent, e))
    })?;
    
    let id = FsLookup(&mb, parent_id, name).ok_or_else(||
    {
        Error::new(ErrorKind::NotFound, format!("`{}` not found", path))
    })?;
    
    let stat = FsStat(&mb, id).ok_or_else(||
    {
        Error::new(ErrorKind::Other, format!("Can't stat `{}`", path))
    })?;
    
    if stat.kind != Kind::Directory
    {
        return Err(Error::new
        (
            ErrorKind::NotADirectory,
            format!("`{}` is not a directory", path)
        ));
    }
    
    // Check if directory is empty
    let entries = FsListDir(&mb, id);

    if !entries.is_empty()
    {
        return Err(Error::new
        (
            ErrorKind::DirectoryNotEmpty,
            format!("`{}` is not empty", path)
        ));
    }
    
    FsUnlink(&mb, parent_id, name).map_err(|e|
    {
        Error::new(e.into(), format!("Failed to remove `{}`: {:?}", path, e))
    })?;
    
    Ok(())
}

/// Removes a file from the filesystem.
#[status(stable)]
pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()>
{
    let path = path.as_ref();
    
    let (parent, name) = match (path.parent(), path.file_name())
    {
        (Some(p), Some(n)) => (p, n),
        _ => return Err(Error::new(ErrorKind::Other, "Cannot remove root")),
    };
    
    let (parent_id, mb) = FsResolve(parent.as_str()).map_err(|e|
    {
        Error::new(e.into(), format!("Can't resolve parent `{}`: {:?}", parent, e))
    })?;
    
    let id = FsLookup(&mb, parent_id, name).ok_or_else(||
    {
        Error::new(ErrorKind::NotFound, format!("`{}` not found", path))
    })?;
    
    let stat = FsStat(&mb, id).ok_or_else(||
    {
        Error::new(ErrorKind::Other, format!("Can't stat `{}`", path))
    })?;
    
    if stat.kind == Kind::Directory
    {
        return Err(Error::new
        (
            ErrorKind::IsADirectory,
            format!("`{}` is a directory", path)
        ));
    }
    
    FsUnlink(&mb, parent_id, name).map_err(|e|
    {
        Error::new(e.into(), format!("Failed to remove `{}`: {:?}", path, e))
    })?;
    
    Ok(())
}

/// Helper function to recursively remove a directory entry.
fn remove_entry_recursive(mb: &Arc<MetaBlock>, parent_id: InodeId, name: &str, id: InodeId) -> Result<()>
{
    let stat = FsStat(mb, id).ok_or_else(||
    {
        Error::new(ErrorKind::Other, "Can't stat entry")
    })?;
    
    // If it's a directory, recursively remove its contents first
    if stat.kind == Kind::Directory
    {
        let children = FsListDir(mb, id);

        for (child_name, child_id) in children
        {
            remove_entry_recursive(mb, id, &child_name, child_id)?;
        }
    }
    
    // Now unlink the entry itself
    FsUnlink(mb, parent_id, name).map_err(|e|
    {
        Error::new(e.into(), format!("Failed to unlink `{}`: {:?}", name, e))
    })?;
    
    Ok(())
}

/// Removes a directory at this path, after removing all its contents. Use carefully!
///
/// This function does **not** follow symbolic links and it will simply remove the
/// symbolic link itself.
#[status(stable)]
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()>
{
    let path = path.as_ref();
    
    let (id, mb) = FsResolve(path.as_str()).map_err(|e|
    {
        Error::new(e.into(), format!("Can't resolve `{}`: {:?}", path, e))
    })?;
    
    let stat = FsStat(&mb, id).ok_or_else(||
    {
        Error::new(ErrorKind::Other, format!("Can't stat `{}`", path))
    })?;
    
    // If it's a directory, remove all its contents first
    if stat.kind == Kind::Directory
    {
        let children = FsListDir(&mb, id);

        for (child_name, child_id) in children
        {
            remove_entry_recursive(&mb, id, &child_name, child_id)?;
        }
    }
    
    // Now remove the top-level entry itself
    let (parent, name) = match (path.parent(), path.file_name())
    {
        (Some(p), Some(n)) => (p, n),
        _ => return Err(Error::new(ErrorKind::Other, "Cannot remove root")),
    };
    
    let (parent_id, parent_mb) = FsResolve(parent.as_str()).map_err(|e|
    {
        Error::new(e.into(), format!("Can't resolve parent `{}`: {:?}", parent, e))
    })?;
    
    FsUnlink(&parent_mb, parent_id, name).map_err(|e|
    {
        Error::new(e.into(), format!("Failed to remove `{}`: {:?}", path, e))
    })?;
    
    Ok(())
}

/// Rename a file or directory to a new name, replacing the original file if `to` already exists.
///
/// This implementation uses link + unlink semantics, which works correctly for regular files.
/// For directories, this requires a dedicated `FsRename` system call which is not yet available.
#[status(unstable)]
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()>
{
    let from = from.as_ref();
    let to = to.as_ref();
    
    let (from_parent, from_name) = match (from.parent(), from.file_name())
    {
        (Some(p), Some(n)) => (p, n),
        _ => return Err(Error::new(ErrorKind::Other, "Cannot rename root")),
    };
    
    let (to_parent, to_name) = match (to.parent(), to.file_name())
    {
        (Some(p), Some(n)) => (p, n),
        _ => return Err(Error::new(ErrorKind::Other, "Cannot rename to root")),
    };
    
    // Resolve the source
    let (from_parent_id, from_mb) = FsResolve(from_parent.as_str()).map_err(|e|
    {
        Error::new(e.into(), format!("Can't resolve source parent `{}`: {:?}", from_parent, e))
    })?;
    
    let from_id = FsLookup(&from_mb, from_parent_id, from_name).ok_or_else(||
    {
        Error::new(ErrorKind::NotFound, format!("Source `{}` not found", from))
    })?;
    
    let from_stat = FsStat(&from_mb, from_id).ok_or_else(||
    {
        Error::new(ErrorKind::Other, format!("Can't stat source `{}`", from))
    })?;
    
    // We cannot rename directories without a proper FsRename system call
    if from_stat.kind == Kind::Directory
    {
        return Err(Error::new
        (
            ErrorKind::Other,
            format!("Renaming directories is not supported: `{}`", from)
        ));
    }
    
    // Resolve the destination parent
    let (to_parent_id, to_mb) = FsResolve(to_parent.as_str()).map_err(|e|
    {
        Error::new(e.into(), format!("Can't resolve destination parent `{}`: {:?}", to_parent, e))
    })?;
    
    // Check if destination already exists and remove it
    if let Some(existing_id) = FsLookup(&to_mb, to_parent_id, to_name)
    {
        let existing_stat = FsStat(&to_mb, existing_id);

        if let Some(stat) = existing_stat
        {
            if stat.kind == Kind::Directory
            {
                return Err(Error::new
                (
                    ErrorKind::IsADirectory,
                    format!("Destination `{}` is a directory", to)
                ));
            }

            // Remove existing file
            FsUnlink(&to_mb, to_parent_id, to_name).map_err(|e|
            {
                Error::new(e.into(), format!("Failed to remove existing destination: {:?}", e))
            })?;
        }
    }
    
    // Create a hard link at the destination
    FsLink(&to_mb, to_parent_id, to_name, from_id).map_err(|e|
    {
        Error::new(e.into(), format!("Failed to create link at destination: {:?}", e))
    })?;
    
    // Remove the original
    FsUnlink(&from_mb, from_parent_id, from_name).map_err(|e|
    {
        // Try to clean up the destination if unlink fails
        let _ = FsUnlink(&to_mb, to_parent_id, to_name);
        Error::new(e.into(), format!("Failed to unlink source: {:?}", e))
    })?;
    
    Ok(())
}

/// Changes the permissions found on a file or folder.
///
/// # Status: Incomplete
///
/// This function requires a `FsChmod` system call which is not yet available in the kernel.
#[status(incomplete)]
pub fn set_permissions<P: AsRef<Path>>(_path: P, _perm: Permissions) -> Result<()>
{
    unimplemented!("set_permissions requires FsChmod system call which is not yet available")
}

/// Changes the permissions found on a file or folder, without following symbolic links.
///
/// # Status: Incomplete
///
/// This function requires a `FsLchmod` system call which is not yet available in the kernel.
#[status(incomplete)]
pub fn set_permissions_nofollow<P: AsRef<Path>>(_path: P, _perm: Permissions) -> Result<()>
{
    unimplemented!("set_permissions_nofollow requires FsLchmod system call which is not yet available")
}

/// Changes the timestamps of the file or folder.
///
/// # Status: Incomplete
///
/// This function requires a `FsSetTimes` system call which is not yet available in the kernel.
#[status(incomplete)]
pub fn set_times<P: AsRef<Path>>(_path: P, _times: FileTimes) -> Result<()>
{
    unimplemented!("set_times requires FsSetTimes system call which is not yet available")
}

/// Changes the timestamps of the file or folder, without following symbolic links.
///
/// # Status: Incomplete
///
/// This function requires a `FsLutimes` system call which is not yet available in the kernel.
#[status(incomplete)]
pub fn set_times_nofollow<P: AsRef<Path>>(_path: P, _times: FileTimes) -> Result<()>
{
    unimplemented!("set_times_nofollow requires FsLutimes system call which is not yet available")
}

/// Creates a new symbolic link on the filesystem.
///
/// # Status: Incomplete
///
/// This function requires a `FsSymlink` system call (or a way to set symlink target via `FsNew`)
/// which is not yet available in the kernel.
#[status(incomplete)]
pub fn soft_link<P: AsRef<Path>, Q: AsRef<Path>>(_original: P, _link: Q) -> Result<()>
{
    unimplemented!("soft_link requires FsSymlink system call which is not yet available")
}
