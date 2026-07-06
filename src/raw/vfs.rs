//! Virtual File System (VFS) abstractions and raw bindings.
//!
//! This module defines the core VFS data structures, including inodes ([`Inode`]),
//! file types ([`Kind`]), permissions ([`InodeFlags`]), and the [`FileSystem`] trait
//! for implementing custom file systems. It also provides raw system calls for
//! VFS operations and meta-block management.

use core::io::ErrorKind;

use alloc::string::String;
use alloc::sync::Arc;
use crate::*;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum FsError {
    Unknown         = 0,
    NotAFile        = 1,
    OutOfBounds     = 2,
    NoEntry         = 3,
    NotADirectory   = 4,
    Found           = 5,
    AlreadyExists   = 6,
    InvalidPath     = 7,
    NotMounted      = 8,
    NotEmpty        = 9,
}

impl Into<ErrorKind> for FsError {
    fn into(self) -> ErrorKind {
        match self {
            FsError::Unknown => ErrorKind::Other,
            FsError::NotAFile => ErrorKind::IsADirectory,
            FsError::OutOfBounds => ErrorKind::InvalidInput,
            FsError::NoEntry => ErrorKind::NotFound,
            FsError::NotADirectory => ErrorKind::NotADirectory,
            FsError::Found => ErrorKind::AlreadyExists,
            FsError::AlreadyExists => ErrorKind::AlreadyExists,
            FsError::InvalidPath => ErrorKind::InvalidFilename,
            FsError::NotMounted => ErrorKind::NotConnected,
            FsError::NotEmpty => ErrorKind::DirectoryNotEmpty,
        }
    }
}

extrum::extrum! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum InodeFlags: u64 {
        DIR         = 1 << 0    ,
        USER_READ   = 1 << 1    ,
        USER_WRITE  = 1 << 2    ,
        USER_EXEC   = 1 << 3    ,
        GROUP_READ  = 1 << 4    ,
        GROUP_WRITE = 1 << 5    ,
        GROUP_EXEC  = 1 << 6    ,
        OTHER_READ  = 1 << 7    ,
        OTHER_WRITE = 1 << 8    ,
        OTHER_EXEC  = 1 << 9    ,
        LEVEL_READ  = 1 << 10   ,
        LEVEL_WRITE = 1 << 11   ,
        LEVEL_EXEC  = 1 << 12   ,
    }
}

impl core::ops::BitOr for InodeFlags { type Output = Self; fn bitor(self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) } }
impl core::ops::BitAnd for InodeFlags { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) } }
impl core::ops::Not for InodeFlags { type Output = Self; fn not(self) -> Self::Output { Self(!self.0) } }

impl InodeFlags {
    pub fn level(self) -> u16 { (self.0 >> 48) as u16 }
    pub fn set_level(&mut self, level: u16) {
        self.0 &= !0 << 16 >> 16;
        self.0 |= (level as u64) << 48;
    }
}

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InodeId(pub u32, pub u32); // (inode number, metablock id)

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Unknown     = 0,
    File        = 1,
    Directory   = 2,
    Socket      = 3,
    Virtual     = 4,
    SymLink     = 5,
}

#[repr(C, align(128))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Inode {
    pub id      : InodeId       ,
    pub kind    : Kind          ,
    pub flags   : InodeFlags  ,
    pub size    : u64           ,
    pub uid     : u16           ,
    pub gid     : u16           ,
    pub atime   : u64           ,
    pub mtime   : u64           ,
    pub ctime   : u64           ,
    pub nlink   : u32           ,
    pub private : [u8; 34]      ,
}

impl Default for Inode {
    fn default() -> Self {
        Self {
            id      : InodeId(0, 0)     ,
            kind    : Kind::Unknown     ,
            flags   : InodeFlags::from_raw(0),
            size    : 0                 ,
            uid     : 0                 ,
            gid     : 0                 ,
            atime   : 0                 ,
            mtime   : 0                 ,
            ctime   : 0                 ,
            nlink   : 0                 ,
            private : [0u8; 34]         ,
        }
    }
}

impl Inode {
    pub const fn new() -> Self {
        Self {
            id      : InodeId(0, 0)     ,
            kind    : Kind::Unknown     ,
            flags   : InodeFlags::from_raw(0),
            size    : 0                 ,
            uid     : 0                 ,
            gid     : 0                 ,
            atime   : 0                 ,
            mtime   : 0                 ,
            ctime   : 0                 ,
            nlink   : 0                 ,
            private : [0u8; 34]         ,
        }
    }
}

pub trait FileSystem: Send + Sync {
    fn lookup(
        &self   ,
        dir     : InodeId,
        name    : &str
    )   ->      Option<InodeId>
    ;
    fn readdir(
        &self   ,
        dir     : InodeId,
        offset  : usize
    )   ->      Option<(String, InodeId)>
    ;
    fn read(
        &self   ,
        file    : InodeId,
        offset  : usize,
        buf     : &mut [u8]
    )   ->      Result<usize, FsError>
    ;
    fn read_link(
        &self   ,
        file    : InodeId,
        offset  : usize,
        buf     : &mut [u8]
    )   ->      Result<usize, FsError>
    {
        let _ = (file, offset, buf);
        Err(FsError::Unknown)
    }
    fn write(
        &self   ,
        file    : InodeId,
        offset  : usize,
        buf     : &[u8]
    )   ->      Result<usize, FsError>
    ;
    fn truncate(
        &self   ,
        file    : InodeId,
        new_size: usize
    )   ->      Result<(), FsError>
    ;
    fn unlink(
        &self   ,
        dir     : InodeId,
        name    : &str
    )   ->      Result<(), FsError>
    ;
    fn link(
        &self   ,
        parent  : InodeId,
        name    : &str,
        child   : InodeId
    )   ->      Result<(), FsError>
    ;
    fn new(
        &self   ,
        mb_id   : u32,
        inode   : Inode,
        kind    : Kind
    )   ->      Result<InodeId, FsError>
    ;
    fn stat(
        &self   ,
        inode   : InodeId
    )   ->      Option<Inode>
    ;
    fn set_mb_id(
        &self   ,
        mb_id   : u32
    )
    ;
    fn probe_blkdev(
        &self   ,
        device  : String
    )   ->      bool
    {   let _   =
        device  ;
        false   }
}

pub type MetaBlockId = u32;

pub struct MetaBlock {
    pub id: MetaBlockId,
    pub fs: Arc<dyn FileSystem>,
}

impl MetaBlock {
    pub const fn new(id: MetaBlockId, fs: Arc<dyn FileSystem>) -> Self {
        MetaBlock { id, fs }
    }
}

Import! {
    pub fn FsLookup(mb: &MetaBlock, dir: InodeId, name: &str) -> Option<InodeId> where kernel 0.1;

    pub fn FsReaddir(mb: &MetaBlock, dir: InodeId, offset: usize) -> Option<(String, InodeId)> where kernel 0.1;

    pub fn FsRead(mb: &MetaBlock, file: InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, FsError> where kernel 0.1;

    pub fn FsReadLink(mb: &MetaBlock, file: InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, FsError> where kernel 0.1;

    pub fn FsWrite(mb: &MetaBlock, file: InodeId, offset: usize, buf: &[u8]) -> Result<usize, FsError> where kernel 0.1;

    pub fn FsTrunc(mb: &MetaBlock, file: InodeId, new_size: usize) -> Result<(), FsError> where kernel 0.1;

    pub fn FsUnlink(mb: &MetaBlock, dir: InodeId, name: &str) -> Result<(), FsError> where kernel 0.1;

    pub fn FsLink(mb: &MetaBlock, parent: InodeId, name: &str, child: InodeId) -> Result<(), FsError> where kernel 0.1;

    pub fn FsNew(mb: &MetaBlock, inode: Inode, kind: Kind) -> Result<InodeId, FsError> where kernel 0.1;

    pub fn FsStat(mb: &MetaBlock, inode: InodeId) -> Option<Inode> where kernel 0.1;

    pub fn FsIsMountPoint(id: InodeId) -> bool where kernel 0.1;

    pub fn FsResolve(path: &str) -> Result<(InodeId, Arc<MetaBlock>), FsError> where kernel 0.1;

    pub fn FsRegMblock(fs: Arc<dyn FileSystem>) -> u32 where kernel 0.1;

    pub fn FsGetMblock(id: u32) -> Option<Arc<MetaBlock>> where kernel 0.1;

    pub fn FsListDir(mb: &MetaBlock, dir: InodeId) -> alloc::collections::btree_map::BTreeMap<String, InodeId> where kernel 0.1;

    pub fn FsReadToString(mb: &MetaBlock, file: InodeId) -> Result<String, FsError> where kernel 0.1;

    pub fn FsMount(name: String, mb: u32) -> Option<InodeId> where kernel 0.1;

    pub fn FsCanonicalize(path: &str) -> Result<String, FsError> where kernel 0.1;
}
