use crate::Vertex;

pub struct Mesh {
    pub verts: &'static [Vertex],
    pub vert_normals: bool,
    pub vert_texcoords: bool,
    pub indices: &'static [u16],
    pub color: [f32; 3],
}
impl Mesh {}
