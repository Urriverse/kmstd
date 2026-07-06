use core::io::{Error, ErrorKind};
use alloc::{format, string::ToString, sync::Arc, vec::Vec};
pub use core::io::Result;

use alloc::{borrow::ToOwned, string::String};

use crate::raw::*;

pub type Path = String;

pub type Time = u64;

pub struct Metadata(Inode);
pub struct FileType(Kind);
pub struct Permissions { readonly: bool }

pub struct DirEntry(Inode, Path);

pub struct ReadDir(Arc<MetaBlock>, InodeId, usize, Path);

impl Iterator for ReadDir {
    type Item = DirEntry;
    fn next(&mut self) -> Option<Self::Item> {
        let (name, id) = FsReaddir(&self.0, self.1, self.2)?;
        self.2 += 1;
        Some(DirEntry(FsStat(&self.0, id)?, self.3.clone() + "/" + name.as_str()))
    }
}

impl DirEntry {
    pub fn path(&self) -> Path { self.1.clone() }
    pub fn metadata(&self) -> Result<Metadata> { Ok(Metadata(self.0)) }
    pub fn file_type(&self) -> Result<FileType> { Ok(FileType(self.0.kind)) }
    pub fn file_name(&self) -> String { self.1.split('/').last().unwrap().to_string() }
}

impl FileType {
    pub fn      is_dir(&self) ->        bool { self.0 == Kind::Directory }
    pub fn     is_file(&self) ->        bool { self.0 == Kind::File      }
    pub fn  is_symlink(&self) ->        bool { self.0 == Kind::SymLink   }
}

impl Permissions {
    pub fn readonly(&self) -> bool {
        self.readonly
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        self.readonly = readonly
    }
}

impl Metadata {
    pub fn   file_type(&self) ->    FileType { FileType(self.0.kind)          }
    pub fn      is_dir(&self) ->        bool { self.0.kind == Kind::Directory }
    pub fn     is_file(&self) ->        bool { self.0.kind == Kind::File      }
    pub fn  is_symlink(&self) ->        bool { self.0.kind == Kind::SymLink   }
    pub fn         len(&self) ->         u64 { self.0.size                    }
    pub fn permissions(&self) -> Permissions {
        Permissions {
            readonly: self.0.flags &
            (   InodeFlags::USER_READ
            |   InodeFlags::GROUP_READ
            |   InodeFlags::OTHER_READ
            )   !=  0
        }
    }
}

fn path_parent(path: &Path) -> Option<Path> {
    path.rsplitn(2, "/").last().map(|p|p.to_owned())
}

pub fn canonicalize(path: &Path) -> Result<Path> {
    match FsCanonicalize(path.as_str()) {
        Ok(path) => Ok(path),
        Err(e) => Err(Error::new(e.into(), format!("Can't canonicalize `{}`: {:?}", path, e)))
    }
}

pub fn create_dir(path: &Path) -> Result<()> {
    let ppath
    =   path_parent(path)
    .   ok_or_else(
        || Error::new(
            ErrorKind::NotFound,
            "Attempt to create root"
        )
    )?;

    let (id, mb)
    =   FsResolve(&ppath)
    .   map_err(
        |e| Error::new(
            e.into(),
            format!("Can't resolve `{}`: {:?}", ppath, e)
        )
    )?;

    let stat
    =   FsStat(&mb, id)
    .   ok_or_else(
        || Error::new(
            ErrorKind::Other,
            format!("Can't stat `{}`", ppath)
        )
    )?;

    if stat.kind != Kind::Directory {
        return Err(
            Error::new(
                ErrorKind::NotADirectory,
                format!("`{}` is not a directory", ppath)
            )
        )
    }

    FsNew(&mb, stat, Kind::Directory)
    .   map_err(
        |e| Error::new(
            e.into(),
            alloc::format!("Error: {:?}", e)
        )
    )?;

    Ok(())
}

pub fn create_dir_all(path: &Path) -> Result<()> {
    match create_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    match path_parent(path) {
                        Some(path) => create_dir_all(&path),
                        None => Err(Error::new(
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

pub fn exists(path: &Path) -> Result<bool> {
    match FsResolve(path.as_str()) {
        Ok(_) => Ok(true),
        Err(e) => match e {
            FsError::NoEntry => Ok(false),
            _ => Err(Error::new(e.into(), format!("Error: {:?}", e)))
        }
    }
}

pub fn metadata(path: &Path) -> Result<Metadata> {
    match FsResolve(path.as_str()) {
        Ok((id, mb)) => {
            match FsStat(&mb, id) {
                Some(inode) => Ok(Metadata(inode)),
                None => Err(Error::new(ErrorKind::Other, format!("Can't stat `{}`", path))),
            }
        },
        Err(e) => Err(Error::new(e.into(), format!("Error: {:?}", e))),
    }
}

pub fn modified(path: &Path) -> Result<Time> {
    Ok(metadata(path)?.0.mtime)
}

pub fn accessed(path: &Path) -> Result<Time> {
    Ok(metadata(path)?.0.atime)
}

pub fn created(path: &Path) -> Result<Time> {
    Ok(metadata(path)?.0.ctime)
}

pub fn read(path: &Path) -> Result<Vec<u8>> {
    match FsResolve(path.as_str()) {
        Ok((id, mb)) => {
            match FsStat(&mb, id) {
                Some(inode) => {
                    if inode.kind != Kind::File {
                        return Err(Error::new(ErrorKind::IsADirectory, format!("Can't read `{}`: Not a file", path)))
                    }
                    let mut buf = [0u8].repeat(inode.size as _);
                    match FsRead(&mb, id, 0, &mut buf) {
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

pub fn read_dir(path: &Path) -> Result<ReadDir> {
    match FsResolve(path.as_str()) {
        Ok((id, mb)) => {
            match FsStat(&mb, id) {
                Some(inode) => {
                    if inode.kind != Kind::Directory {
                        return Err(Error::new(ErrorKind::NotADirectory, format!("Can't read dir `{}`: Not a directory", path)))
                    }
                    Ok(ReadDir(mb, id, inode.size as _, path.clone()))
                },
                None => Err(Error::new(ErrorKind::Other, format!("Can't stat `{}`", path))),
            }
        },
        Err(e) => Err(Error::new(e.into(), format!("Can't resolve `{}`: {:?}", path, e))),
    }
}

pub fn read_link(path: &Path) -> Result<Path> {
    match FsResolve(path.as_str()) {
        Ok((id, mb)) => {
            match FsStat(&mb, id) {
                Some(inode) => {
                    if inode.kind != Kind::SymLink {
                        return Err(Error::new(ErrorKind::IsADirectory, format!("Can't read symlink `{}`: Not a symlink", path)))
                    }
                    let mut buf = [0u8].repeat(inode.size as _);
                    match FsReadLink(&mb, id, 0, &mut buf) {
                        Ok(_) => Ok(String::from_utf8_lossy_owned(buf)),
                        Err(e) => Err(Error::new(e.into(), format!("Failed to read `{}`: {:?}", path, e)))
                    }
                },
                None => Err(Error::new(ErrorKind::Other, format!("Can't stat `{}`", path)))
            }
        },
        Err(e) => Err(Error::new(e.into(), format!("Can't resolve `{}`: {:?}", path, e)))
    }
}

pub fn read_to_string(path: &Path) -> Result<String> {
    Ok(String::from_utf8_lossy(&read(path)?).to_string())
}

pub fn write(path: &Path, contents: &[u8]) -> Result<()> {
    match FsResolve(path.as_str()) {
        Ok((id, mb)) => {
            match FsStat(&mb, id) {
                Some(inode) => {
                    if inode.kind != Kind::File {
                        return Err(Error::new(ErrorKind::IsADirectory, format!("Can't read `{}`: Not a file", path)))
                    }
                    match FsWrite(&mb, id, 0, contents) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(Error::new(e.into(), format!("Failed to read `{}`: {:?}", path, e)))
                    }
                },
                None => Err(Error::new(ErrorKind::Other, format!("Can't stat `{}`", path)))
            }
        },
        Err(e) => Err(Error::new(e.into(), format!("Can't resolve `{}`: {:?}", path, e)))
    }
}

// TODO:
// - remove_dir
// - remove_dir_all
// - remove_file
// - rename
// - set_permissions
// - set_permissions_nofollow
// - set_times
// - set_times_nofollow
// - soft_link
// - symlink_metadata
