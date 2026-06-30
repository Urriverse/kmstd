use ketypes::*;

use crate::*;

#[repr(C, align(128))] pub struct KeDevice {}

impl KeDevice {
    pub fn new(name: KeStr) -> Option<Box![Self]> { KeVtDeviceNew(name) }
}
