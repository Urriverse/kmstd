pub macro panic_handler(){#[panic_handler]fn __(x:&core::panic::PanicInfo)->!{crate::ExecPanic(x)}}
pub macro exit{()=>{crate::ExecExit(0)},($code:expr)=>{crate::ExecExit($code)},}
