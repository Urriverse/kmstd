use crate::*;

Import! {
    pub fn VmTryMap(va: Vaddr, pa: Paddr, size: usize, flags: EntryFlags) -> Result<(), &'static str> where kernel 0.1;

    pub fn VmTryRemap(va: usize, size: usize, new_flags: EntryFlags) -> Result<(), &'static str> where kernel 0.1;

    pub fn VmTryUnmap(va: usize, size: usize) -> Result<(), &'static str> where kernel 0.1;

    pub fn VmMergeRange(start: usize, size: usize) where kernel 0.1;

    pub fn VmQuery(va: usize) -> Option<(Paddr, EntryFlags)> where kernel 0.1;
}
