pub macro SYMBOL( $( $v:vis $n:ident : $t:ty = $d:expr; )+ ) {
    $(
        #[used]
        #[unsafe(no_mangle)]
        $v static $n: $t = $d;
    )*
}

pub macro entry( mod $n:literal ; $( $b:tt )* ) {
    #[cfg(not(test))]
    panic_handler!{}
    
    SYMBOL! {
        pub MODNAME:&'static str=$n;
    }
    
    #[unsafe(no_mangle)]
    pub extern "C" fn _start() {
        $($b)*
    }
}
