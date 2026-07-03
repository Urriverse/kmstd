use alloc::string::String;
use alloc::sync::Arc;

#[repr(usize)]
#[derive(Debug)]
pub enum KeFsError {
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

extrum::extrum! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum KeInodeFlags: u64 {
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

impl KeInodeFlags {
    pub fn level(self) -> u16 { (self.0 >> 48) as u16 }
    pub fn set_level(&mut self, level: u16) {
        self.0 &= !0 << 16 >> 16;
        self.0 |= (level as u64) << 48;
    }
}

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeInodeId(pub u32, pub u32); // (inode number, metablock id)

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
pub struct KeInode {
    pub id      : KeInodeId   ,
    pub kind    : Kind      ,
    pub flags   : KeInodeFlags     ,
    pub size    : u64       ,
    pub uid     : u16       ,
    pub gid     : u16       ,
    pub atime   : u64       ,
    pub mtime   : u64       ,
    pub ctime   : u64       ,
    pub nlink   : u32       ,
    pub private : [u8; 34]  ,
}

impl Default for KeInode {
    fn default() -> Self {
        Self {
            id      : KeInodeId(0, 0)     ,
            kind    : Kind::Unknown     ,
            flags   : KeInodeFlags::from_raw(0),
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

impl KeInode {
    pub const fn new() -> Self {
        Self {
            id      : KeInodeId(0, 0)     ,
            kind    : Kind::Unknown     ,
            flags   : KeInodeFlags::from_raw(0),
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

pub trait KeFileSystem: Send + Sync {
    fn lookup(
        &self   ,
        dir     : KeInodeId,
        name    : &str
    )   ->      Option<KeInodeId>
    ;
    fn readdir(
        &self   ,
        dir     : KeInodeId,
        offset  : usize
    )   ->      Option<(String, KeInodeId)>
    ;
    fn read(
        &self   ,
        file    : KeInodeId,
        offset  : usize,
        buf     : &mut [u8]
    )   ->      Result<usize, KeFsError>
    ;
    fn write(
        &self   ,
        file    : KeInodeId,
        offset  : usize,
        buf     : &[u8]
    )   ->      Result<usize, KeFsError>
    ;
    fn truncate(
        &self   ,
        file    : KeInodeId,
        new_size: usize
    )   ->      Result<(), KeFsError>
    ;
    fn unlink(
        &self   ,
        dir     : KeInodeId,
        name    : &str
    )   ->      Result<(), KeFsError>
    ;
    fn link(
        &self   ,
        parent  : KeInodeId,
        name    : &str,
        child   : KeInodeId
    )   ->      Result<(), KeFsError>
    ;
    fn new(
        &self   ,
        mb_id   : u32,
        inode   : KeInode,
        kind    : Kind
    )   ->      Result<KeInodeId, KeFsError>
    ;
    fn stat(
        &self   ,
        inode   : KeInodeId
    )   ->      Option<KeInode>
    ;
    fn set_mb_id(
        &self   ,
        mb_id   : u32
    )
    ;
}

pub type KeMetaBlockId = u32;

pub struct KeMetaBlock {
    pub id: KeMetaBlockId,
    pub fs: Arc<dyn KeFileSystem>,
}

impl KeMetaBlock {
    pub const fn new(id: KeMetaBlockId, fs: Arc<dyn KeFileSystem>) -> Self {
        KeMetaBlock { id, fs }
    }
}
