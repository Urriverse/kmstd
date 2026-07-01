use ketypes::*;

Import! {
    pub fn VtDeviceNew(name: KeStr) -> Option<Box![KeDevice]> where kernel 0.0 {
        let _ = name;
        return None;
    }
}
