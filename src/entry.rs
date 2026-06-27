#[unsafe(no_mangle)]
pub fn _start(st: crate::kst::KeSysTab) {
    (st.suicide)()
}
