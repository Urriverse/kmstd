pub type EntryPoint<'a> = &'a (dyn Fn() -> i32 + Sync + core::panic::RefUnwindSafe);

#[cfg(not(test))]
pub fn lang_start_internal(main: EntryPoint, argc: isize, argv: *const *const u8) -> !
{
    let _ = (argc, argv);

    main();

    crate::raw::ExecExit(0);
}
