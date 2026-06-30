pub macro SYMBOL (
    $(
        $v:vis $n:ident: $t:ty = $d:expr;
    )+
) {
    $(
        #[used]
        #[unsafe(no_mangle)]
        $v static $n: $t = $d;
    )*
}

pub macro meta($n:expr) {
    #[cfg(not(test))] panic_handler![];

    SYMBOL! { pub MODNAME: &'static str = $n; }
}

pub macro entry( $($b:tt)* ) {
    #[unsafe(no_mangle)]
    pub extern "C" fn _start() {
        $(
            $b
        )*
    }
}

pub macro Ku( $( $x:ident or $d:expr )+ ) {
    SYMBOL! {
        $(
            pub $x: $x = $d;
        )+
    }
}
