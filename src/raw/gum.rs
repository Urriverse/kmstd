//! Raw bindings for the Global Uniform Map.
//!
//! This module provides direct access to the underlying GUM data system operations,
//! such as reading, writing, resolving paths, and manipulating nodes without
//! the overhead of the higher-level VFS meta-block abstraction.

use crate::raw::
{
    FsError ,
    InodeId ,
    Inode   ,
    Kind    ,
};

type GumError = FsError;

Import!
{
    pub fn GumLookup(dir: InodeId, name: &str) -> Option<InodeId>
    where kernel 0.1;

    pub fn GumReaddir(dir: InodeId, offset: usize) -> Option<(String, InodeId)>
    where kernel 0.1;

    pub fn GumRead(file: InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, GumError>
    where kernel 0.1;

    pub fn GumWrite(file: InodeId, offset: usize, buf: &[u8]) -> Result<usize, GumError>
    where kernel 0.1;

    pub fn GumTrunc(file: InodeId, new_size: usize) -> Result<(), GumError>
    where kernel 0.1;

    pub fn GumUnlink(dir: InodeId, name: &str) -> Result<(), GumError>
    where kernel 0.1;

    pub fn GumLink(parent: InodeId, name: &str, child: InodeId) -> Result<(), GumError>
    where kernel 0.1;

    pub fn GumNew(inode: Inode, kind: Kind) -> Result<InodeId, GumError>
    where kernel 0.1;

    pub fn GumStat(inode: InodeId) -> Option<Inode>
    where kernel 0.1;

    pub fn GumResolve(path: &str) -> Result<InodeId, GumError>
    where kernel 0.1;

    pub fn GumListDir(dir: InodeId) -> alloc::collections::btree_map::BTreeMap<String, InodeId>
    where kernel 0.1;

    pub fn GumReadToString(file: InodeId) -> Result<String, GumError>
    where kernel 0.1;
}
