pub type Swapper = *const fn(pfn: usize) -> Result<(), ()>;

#[derive(Clone, Copy, Debug)] #[repr(transparent)] pub struct Paddr(usize);
#[derive(Clone, Copy, Debug)] #[repr(transparent)] pub struct Vaddr(usize);

impl Paddr {
    #[inline(always)] pub const fn from_raw(r: usize) -> Self { Self(r) }

    #[inline(always)] pub const fn to_raw(self) -> usize { self.0 }
}

impl Vaddr {
    #[inline(always)] pub const fn from_raw(r: usize) -> Self { Self(r) }

    #[inline(always)] pub fn from_ptr<T>(ptr: *const T) -> Self { Self::from_raw(ptr as usize) }

    #[inline(always)] pub fn from_ptr_mut<T>(ptr: *mut T) -> Self { Self::from_raw(ptr as usize) }

    #[inline(always)] pub fn from_ref<T>(r: &'_ T) -> Self { Self::from_ptr(r) }

    #[inline(always)]
    pub fn from_ref_mut<T>(r: &'_ mut T) -> Self { Self::from_ptr_mut(r) }

    #[inline(always)] pub const fn to_raw(self) -> usize { self.0 }

    #[inline(always)] pub const fn to_ptr<T>(self) -> *const T { self.0 as *const T }

    #[inline(always)] #[allow(clippy::wrong_self_convention)] pub const fn to_ptr_mut<T>(self) -> *mut T { self.0 as *mut T }

    #[inline(always)] pub const fn to_ref<'a, T>(self) -> Option<&'a T> { unsafe { self.to_ptr::<T>().as_ref() } }
    #[inline(always)] pub const unsafe fn to_ref_unchecked<'a, T>(self) -> &'a T { unsafe { self.to_ptr::<T>().as_ref_unchecked() } }

    #[inline(always)] #[allow(clippy::wrong_self_convention)]
    pub const fn to_mut<'a, T>(self) -> Option<&'a mut T> { unsafe { self.to_ptr_mut::<T>().as_mut() } }

    #[inline(always)] #[allow(clippy::wrong_self_convention)]
    pub const unsafe fn to_mut_unchecked<'a, T>(self) -> &'a mut T { unsafe { self.to_ptr_mut::<T>().as_mut_unchecked() } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Zone {
    Dma = 0,
    Dma32 = 1,
    Normal = 2,
}

impl Zone {
    pub const DMA_END: usize = 16 * 1024 * 1024;      // 16 MiB
    pub const DMA32_END: usize = 4 * 1024 * 1024 * 1024; // 4 GiB

    #[inline]
    pub fn from_pfn(pfn: usize) -> Self {
        let paddr = pfn * 4096;
        if paddr < Self::DMA_END {
            Zone::Dma
        } else if paddr < Self::DMA32_END {
            Zone::Dma32
        } else {
            Zone::Normal
        }
    }

    #[inline]
    pub const fn index(self) -> usize {
        match self {
            Zone::Dma => 0,
            Zone::Dma32 => 1,
            Zone::Normal => 2,
        }
    }
}

Import! {
    pub fn MemAlloc(layout: core::alloc::Layout) -> *mut u8 where kernel 0.1;
    pub fn MemFree(ptr: *mut u8, layout: core::alloc::Layout) where kernel 0.1;
    pub fn MemAllocDMA(zone: Zone, count: usize) -> Paddr where kernel 0.1;
    pub fn MemFreeDMA(paddr: Paddr) where kernel 0.1;
    pub fn MemSetSwapper(swapper: Swapper) where kernel 0.1;
    pub fn MemGetSwapper() -> Swapper where kernel 0.1;
}
