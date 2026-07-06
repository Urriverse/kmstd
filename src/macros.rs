pub macro exit{()=>{crate::ExecExit(0)},($code:expr)=>{crate::ExecExit($code)},}
