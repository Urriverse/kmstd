use crate::*; use ketypes::*;

Ke!
{
    KeVtDeviceNew           or  |_| { error!("KeVtDeviceNew not provided"); None}

    KeDeviceAddMethod       or  |_,_,_| ()

    KeDeviceGetMethod       or  |_,_|   None

    KeDeviceRegister        or  |_|     None

    KeDeviceUnregister      or  |_|     false

    KeDeviceDataGet         or  |_|     None

    KeDeviceDataSet         or  |_,_|   false

    KeDeviceMethodInvoke    or  |_,_,_| KeDeviceResult { value: 0usize, status: KeDeviceStatus::UNSUPPORTED }
}
