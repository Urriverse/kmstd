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
    pub macro mod_ident() {
        $n
    }
}

pub macro entry( $($b:tt)* ) {
    #[unsafe(no_mangle)]
    pub extern "C" fn _start() {
        $(
            $b
        )*
    }
}
