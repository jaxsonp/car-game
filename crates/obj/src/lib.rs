mod mtl;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use crate::mtl::parse_mtl_file;

/// How a vertex is identified in a face definition, e.g. `4/4/3`
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct VertIndexes {
    pos_index: usize,
    // normal_index: usize,
    // texcoord_index: usize,
}

#[derive(PartialEq)]
pub struct Vert {
    pub pos: [f32; 3],
}
type Face = [usize; 3];

pub struct OBJMesh {
    pub verts: Vec<Vert>,
    pub faces: Vec<Face>,
}
impl Default for OBJMesh {
    fn default() -> Self {
        Self {
            verts: Vec::new(),
            faces: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct OBJMaterial {
    pub name: String,
    pub diffuse_color: [f32; 3],
}
impl OBJMaterial {
    pub(self) fn new(name: String) -> Self {
        Self {
            name,
            diffuse_color: [1.0, 1.0, 1.0],
        }
    }
}
impl Default for OBJMaterial {
    fn default() -> Self {
        Self {
            name: String::new(),
            diffuse_color: [1.0, 0.0, 1.0],
        }
    }
}

/// Reads in a Wavefront .obj file.
///
/// This parser only supports the following:
/// - Geometric vertices~~, texture coordinates,~~ and normals
/// - Diffuse color in mtllib materials
/// - Triangular faces (poly faces can be read, but will just be parsed as a tri fan, which may produce incorrect results)
///
/// This parser ignores named objects and groups, instead treating the whole file as a model and grouping faces into
/// meshes by material
pub fn parse_obj_file(input_filepath: PathBuf) -> io::Result<Vec<(OBJMaterial, OBJMesh)>> {
    let input_file = File::open(input_filepath.clone())?;
    let input = BufReader::new(input_file);

    let mut raw_vert_positions: Vec<[f32; 3]> = Vec::new();
    //let mut raw_vert_normals: Vec<[f32; 3]> = Vec::new();
    //let mut raw_vert_texcoords: Vec<[f32; 2]> = Vec::new();

    let mut material_table: HashMap<String, usize> = HashMap::new();
    let mut material_list: Vec<OBJMaterial> = Vec::new();

    // hashmap of each material (indexed) and its mesh
    let mut meshes: HashMap<Option<usize>, OBJMesh> = HashMap::new();
    meshes.insert(None, OBJMesh::default());
    let mut current_material_index: Option<usize> = None;
    let mut current_mesh: &mut OBJMesh = meshes.get_mut(&current_material_index).unwrap();

    for line in input.lines() {
        let line = line?;
        let mut words = line.split_ascii_whitespace().map(String::from);
        match words.next().unwrap().as_str() {
            "v" => raw_vert_positions.push(parse_vertex(words)),
            "f" => current_mesh.faces.append(
                &mut parse_faces(words)
                    .into_iter()
                    .map(|verts| {
                        verts.map(|v: VertIndexes| {
                            let new_vert = Vert {
                                pos: raw_vert_positions[v.pos_index],
                            };
                            // checking if this vert already exists (yes I know it is O(n^2) shut up idc)
                            for (index, existing_vert) in current_mesh.verts.iter().enumerate() {
                                if new_vert == *existing_vert {
                                    return index; // vert already exists
                                }
                            }
                            // this is a new vert, emit it
                            current_mesh.verts.push(new_vert);
                            return current_mesh.verts.len() - 1;
                        })
                    })
                    .collect(),
            ),

            "mtllib" => parse_mtl_file(
                input_filepath.parent().unwrap().join(words.next().unwrap()),
                &mut material_list,
                &mut material_table,
            )?,
            "usemtl" => {
                let mtl_name = words.next().unwrap();
                current_material_index = Some(*material_table.get(&mtl_name).unwrap());
                if !meshes.contains_key(&current_material_index) {
                    meshes.insert(current_material_index, OBJMesh::default());
                }
                current_mesh = meshes.get_mut(&current_material_index).unwrap()
            }
            _ => {}
        }
    }

    Ok(meshes
        .into_iter()
        .filter(|(_, mesh)| !mesh.verts.is_empty())
        .map(|(mat_index, mesh)| {
            (
                mat_index
                    .map(|index| material_list[index].clone())
                    .unwrap_or(OBJMaterial::default()),
                mesh,
            )
        })
        .collect())
}

fn parse_vertex<I: Iterator<Item = String>>(mut words: I) -> [f32; 3] {
    let x = words.next().unwrap().parse::<f32>().unwrap();
    let y = words.next().unwrap().parse::<f32>().unwrap();
    let z = words.next().unwrap().parse::<f32>().unwrap();
    let w = if let Some(w) = words.next() {
        w.parse::<f32>().unwrap()
    } else {
        1.0
    };
    return [x / w, y / w, z / w];
}

/// parses a face, treating poly-faces as triangle fans
fn parse_faces<I: Iterator<Item = String>>(mut words: I) -> Vec<[VertIndexes; 3]> {
    let parse_vertex = |s: String| {
        let mut nums = s.splitn(3, '/');
        let vi = nums.next().unwrap().parse::<usize>().unwrap();
        let _vti = nums.next().map(|s| s.parse::<usize>().unwrap());
        let _vni = nums.next().map(|s| s.parse::<usize>().unwrap());
        return VertIndexes { pos_index: vi - 1 };
    };
    let mut first_and_last_vert = None;
    let mut parse_face = || {
        if let Some(word) = words.next() {
            if let Some((first, last)) = first_and_last_vert {
                let face = [first, last, parse_vertex(word)];
                first_and_last_vert = Some((first, face[2]));
                Some(face)
            } else {
                let face = [
                    parse_vertex(word),
                    parse_vertex(words.next().unwrap()),
                    parse_vertex(words.next().unwrap()),
                ];
                first_and_last_vert = Some((face[0], face[2]));
                Some(face)
            }
        } else {
            None
        }
    };

    let mut faces = Vec::new();
    while let Some(new) = parse_face() {
        faces.push(new);
    }
    return faces;
}

//fn parse_mtllib(file)
