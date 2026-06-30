use crate::*; use ketypes::*;

Ku!
{
    KeVtDeviceNew           or  |_|     { error!("KeVtDeviceNew not provided"); None}

    KeDeviceAddMethod       or  |_,_,_| { error!("KeDeviceAddMethod not provided"); }

    KeDeviceGetMethod       or  |_,_|   { error!("KeDeviceGetMethod not provided"); None }

    KeDeviceRegister        or  |_|     { error!("KeDeviceRegister not provided"); None }

    KeDeviceUnregister      or  |_|     { error!("KeDeviceUnregister not provided"); false }

    KeDeviceDataGet         or  |_|     { error!("KeDeviceDataGet not provided"); None }

    KeDeviceDataSet         or  |_,_|   { error!("KeDeviceDataSet not provided"); false }

    KeDeviceMethodInvoke    or  |_,_,_| { error!("KeDeviceMethodInvoke not provided"); KeDeviceResult { value: 0usize, status: KeDeviceStatus::UNSUPPORTED } }
}
