pub macro entry( $($b:tt)* ) {
    #[unsafe(no_mangle)]
    pub extern "C" fn _start() {
        $($b)*
    }
}
