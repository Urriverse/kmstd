use ketypes::*;

Import! {
    pub fn MonLog
    (
        level: KeAttLvl,
        module: KeStr,
        file: KeStr,
        line: u32,
        args: Arguments<'_>
    ) where kernel 0.0 {
        let _ = (level, module, file, line, args);
        /* nothing to do: we can't even log this error */
    }
}
