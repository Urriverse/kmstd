use crate::*; use ketypes::*;

Ku!
{
    KeModuleLoad            or  |_| Err("function not provided".to_string())

    KeModuleSymbols         or  |_| Err("function not provided".to_string())

    KeModuleString          or  |_,_| Err("function not provided".to_string())

    KeModulePointer         or  |_,_| 0 as _

    KeModuleExecute         or  |_| 0
}
