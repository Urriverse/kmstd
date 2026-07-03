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

Import! {
    pub fn MemAlloc(layout: core::alloc::Layout) -> *mut u8 where kernel 0.1;
    pub fn MemFree(ptr: *mut u8, layout: core::alloc::Layout) where kernel 0.1;
}
