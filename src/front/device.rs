use alloc::boxed::Box;

#[repr(C, align(128))] pub struct Device;

extrum::extrum! {
    /// Device operation status codes.
    ///
    /// These are returned by methods as part of `DeviceResult`. A `SUCCESS` status
    /// indicates the operation succeeded; any other value indicates an error.
    #[derive(Clone, Copy, PartialEq)]
    pub enum DeviceStatus: usize {
        SUCCESS = 0,
        NOT_FOUND = 1,
        INVALID_ARG = 2,
        BUSY = 3,
        IO_ERROR = 4,
        UNSUPPORTED = usize::MAX,
    }
}

pub type MethodId = u64;
pub type DeviceMethod = extern "C" fn(DeviceId, usize) -> DeviceResult;
#[derive(Debug, Clone, Copy, PartialEq, Eq)] #[repr(C)] pub struct DeviceId(u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)] #[repr(C)] pub struct DeviceResult { pub value: usize, pub status: DeviceStatus }

impl DeviceId {
    #[inline(always)]
    pub fn invoke(self, mid: MethodId, arg: usize) -> Result<usize, DeviceStatus> { DeviceInvoke(self, mid, arg).as_result() }

    #[inline(always)] pub fn get(self) -> Option<usize> { DeviceGetData(self) }
    #[inline(always)] pub fn set(self, data: usize) -> Result<(), ()> { if DeviceSetData(self, data) { Ok(()) } else { Err(()) } }
    #[inline(always)] pub fn remove(self) -> Option<Box<Device>> { DeviceRemove(self) }
}

impl DeviceResult {
    /// Creates a new result with the given value and status.
    #[inline]
    pub const fn new(value: usize, status: DeviceStatus) -> Self {
        Self { value, status }
    }

    /// Creates a successful result.
    #[inline]
    pub const fn ok(value: usize) -> Self {
        Self { value, status: DeviceStatus::SUCCESS }
    }

    /// Creates an error result with the given status.
    #[inline]
    pub const fn err(status: DeviceStatus) -> Self {
        Self { value: 0, status }
    }

    /// Converts this result into a Rust `Result<usize, DeviceStatus>`.
    #[inline]
    pub fn as_result(self) -> Result<usize, DeviceStatus> {
        if self.status == DeviceStatus::SUCCESS {
            Ok(self.value)
        } else {
            Err(self.status)
        }
    }

    /// Constructs a `DeviceResult` from a Rust `Result`.
    #[inline]
    pub fn from_result(res: Result<usize, DeviceStatus>) -> Self {
        match res {
            Ok(value) => Self::ok(value),
            Err(status) => Self::err(status),
        }
    }
}

impl Device {
    #[inline(always)]
    pub fn new(name: &str) -> Option<Box<Self>> {
        VtDeviceNew(name)
    }

    #[inline(always)]
    pub fn add_method(this: &mut Box<Device>, method_id: MethodId, method: DeviceMethod) {
        VtDeviceAddMethod(this, method_id, method)
    }

    #[inline(always)]
    pub fn get_method(this: &Box<Device>, method_id: MethodId) -> Option<DeviceMethod> {
        VtDeviceGetMethod(this, method_id)
    }
}

Import! {
    pub fn DeviceRegister(device: Box<Device>) -> Option<DeviceId> where kernel 0.1;

    pub fn VtDeviceNew(name: &str) -> Option<Box<Device>> where kernel 0.1;

    pub fn VtDeviceAddMethod(this: &mut Box<Device>, method_id: MethodId, method: DeviceMethod) where kernel 0.1;

    pub fn VtDeviceGetMethod(this: &Box<Device>, method_id: MethodId) -> Option<DeviceMethod> where kernel 0.1;

    pub fn DeviceRemove(id: DeviceId) -> Option<Box<Device>> where kernel 0.1;

    pub fn DeviceSetData(id: DeviceId, data: usize) -> bool where kernel 0.1;

    pub fn DeviceGetData(id: DeviceId) -> Option<usize> where kernel 0.1;

    pub fn DeviceInvoke(id: DeviceId, mid: MethodId, arg: usize) -> DeviceResult where kernel 0.1;
}
