#[macro_export]
macro_rules! load_obj_mesh {
    ($file:literal) => {{
        use crate::{RawMaterial, RawMesh, RawVertex};
        include!(std::concat!(env!("OUT_DIR"), "/", $file, ".rs"))
    }};
}

#[macro_export]
macro_rules! load_font {
    ($name:literal) => {{ include_bytes!(concat!(env!("FONTS_DIR"), "/", $name)) }};
}

#[macro_export]
macro_rules! debug_polyline {
    // base: one point left → done
    (@acc [$($acc:tt)*] $col:expr; $last:expr) => {
        [ $($acc)* ]
    };
    // recursively accumulate
    (@acc [$($acc:tt)*] $col:expr; $prev:expr, $cur:expr $(,$rest:expr)*) => {
        debug_polyline!(@acc [ $($acc)* debug_line!($col, $prev, $cur), ] $col; $cur $(, $rest)* )
    };
    // invocation method
    ($col:expr, $pos0:expr $(, $pos:expr)+ $(,)? ) => {{
        debug_polyline!(@acc [] $col; $pos0 $(,$pos)+ )
    }};
}

#[macro_export]
macro_rules! debug_polyloop {
    ($col:expr, $pos0:expr $(, $pos:expr)+ $(,)? ) => {{
        debug_polyline!($col, $pos0, $($pos,)+ $pos0)
    }};
}

#[macro_export]
macro_rules! debug_line {
    ($col:expr, $pos1:expr, $pos2:expr) => {
        DebugLine {
            col: $col,
            pos1: $pos1,
            pos2: $pos2,
        }
    };
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
