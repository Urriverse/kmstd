use crate::*; use ketypes::*;

Ku!
{
    KeEventSubscribe    or |_,_|    Err(())

    KeEventUnsubscribe  or |_,_|    Err(())

    KeEventPublish      or |_,_,_|  Err(())
}
