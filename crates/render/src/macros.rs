#[macro_export]
macro_rules! load_obj {
    ($file:literal) => {{
        use crate::material::Material;
        use crate::mesh::{RawMesh, Vertex};
        include!(concat!(env!("OUT_DIR"), "/", $file, ".rs"))
    }};
}

#[macro_export]
macro_rules! load_font {
    ($name:literal) => {{ include_bytes!(concat!(env!("FONTS_DIR"), "/", $name)) }};
}
