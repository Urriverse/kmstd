//! Architecture-specific kernel interfaces.
//!
//! This module conditionally compiles and re-exports bindings for the target
//! CPU architecture (e.g., `amd64`).

macro switch( $( $x:ident <= $y:literal )* )
{
    $(
        #[cfg(target_arch = $y)]
        mod $x;

        #[cfg(target_arch = $y)]
        pub use $x::*;
    )*
}

switch!
{
    amd64 <= "x86_64"
}
