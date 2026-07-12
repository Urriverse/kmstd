pub type EntryPoint<'a> = &'a (dyn Fn() -> i32 + Sync + core::panic::RefUnwindSafe);

pub fn lang_start_internal(main: EntryPoint, argc: isize, argv: *const *const u8) -> !
{
    let _ = (argc, argv);

    main();

    crate::raw::ExecExit(0);
}

unsafe extern "Rust"
{
    fn main() -> ();
}

fn entry_point() -> i32
{
    unsafe
    {
        main();
    }
    
    0
}

#[unsafe(no_mangle)]
pub fn _start() -> !
{
    lang_start_internal(&entry_point, 0, 0 as _)
}
