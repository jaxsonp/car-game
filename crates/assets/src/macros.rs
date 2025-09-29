#[macro_export]
macro_rules! preloaded_file {
    ($file:literal) => {{
        #[allow(unused_imports)]
        use crate::{RawMaterial, RawMesh, RawVertex};
        #[allow(unused_imports)]
        use nalgebra::{ArrayStorage, Matrix4};
        include!(std::concat!(env!("OUT_DIR"), "/", $file, ".rs"))
    }};
}

/// Create an array of `DebugLine`s from a set of semicolon-delimited polylines
///
/// Define a polyline like so:
/// ```txt
/// P0 => P1 => P2;
/// ```
#[macro_export]
macro_rules! debug_lines {
    // entry point
    ($($p0:expr $(=> $p:expr)*;)+) => {
        debug_lines!(@acc []; $([$p0 $(, $p)*] )+)
    };

    // more in this line
    (@acc [$($acc:tt)*]; [$pos1:expr, $pos2:expr $(, $rest:expr)*] $($lines:tt)* ) => {
        debug_lines!(@acc [ $($acc)*
            {RawDebugLine {
                col: BLACK,
                pos1: $pos1,
                pos2: $pos2,
            }}
         ]; [$pos2 $(, $rest)*] $($lines)*)
    };

    // no more in this line, but more lines
    (@acc [$($acc:tt)*]; [$pos:expr] $($lines:tt)+ ) => {
        debug_lines!(@acc [$($acc)*]; $($lines)+ )
    };

    // no more period
    (@acc [$($acc:tt)*]; [$pos:expr]) => {
        [ $($acc,)* ]
    };
}
