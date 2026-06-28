#[repr(C)]
pub struct ElfSym {
    pub st_name : u32   ,
    pub st_info : u8    ,
    pub st_other: u8    ,
    pub st_shndx: u16   ,
    pub st_value: u64   ,
    pub st_size : u64   ,
}
