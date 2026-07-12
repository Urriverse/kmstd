use crate::raw::vfs::{MetaBlock, InodeId, FsReaddir, FsStat};
use crate::api::{path::PathBuf, fs::DirEntry};

#[status(stable)]
pub struct ReadDir
(
    pub(crate) sync::Arc<MetaBlock>,
    pub(crate) InodeId,
    pub(crate) usize,
    pub(crate) PathBuf
);

impl Iterator for ReadDir
{
    type Item = DirEntry;

    #[status(stable)]
    fn next(&mut self) -> Option<Self::Item>
    {
        let (name, id) = FsReaddir(&self.0, self.1, self.2)?;
        self.2 += 1;
        let inode = FsStat(&self.0, id)?;
        let mut path = self.3.clone();
        path.push(&name);
        Some(DirEntry(inode, path))
    }
}
