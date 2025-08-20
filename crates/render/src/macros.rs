#[macro_export]
macro_rules! load_obj {
    ($file:literal) => {{
        use crate::material::Material;
        use crate::mesh::RawMesh;
        use crate::vert::Vertex;
        include!(concat!(env!("OUT_DIR"), "/", $file, ".rs"))
    }};
}
