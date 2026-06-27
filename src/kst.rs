pub(crate) type KeSymbolHandle = usize;

pub(crate) type KeSymbolGuard = [usize; 2];

#[repr(C)]
pub(crate) struct KeSymbol {
    mprc: usize,
    rc: usize,
    ptr: usize,
    poisonous: u8,
    _pad: [u8; 3],
    id: u64,
}

#[repr(C)]
pub(crate) struct KeSysTab {
    pub link:               fn(u64) ->  Option<KeSymbolHandle>,
    pub link_guard:         fn(&KeSymbolHandle) -> KeSymbolGuard,
    pub link_guard_get:     fn(&KeSymbolGuard) -> &fn(),
    pub export:             fn(u64, &'static fn()) -> Option<KeSymbol>,
    pub suicide:            fn() -> !,
    pub log:                fn(u8, &'static str, &'static str, u32, *const ()) -> (),
}
