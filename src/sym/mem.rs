use crate::*; use ketypes::*;

Ku!
{
    KeMemAlloc              or  |_|     { error!("KeMemAlloc not provided"); 0 as _}

    KeMemFree               or  |_,_|   { error!("KeMemFree not provided"); }

    KeMemAllocStack         or  |_|     KeVaddr::from_raw(0)

    KeMemVirtToPhys         or  |_|     KePaddr::from_raw(0)

    KeMemPhysToVirt         or  |_|     KeVaddr::from_raw(0)
}
