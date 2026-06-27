pub macro entry( $body:tt ) {
    #[unsafe(no_mangle)]
    pub extern "C" fn _start() {
        $body
    }
}
