use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct VertIndexes {
    vert_index: usize,
}
pub struct Vert {
    pub pos: [f32; 3],
}
pub type Face = [usize; 3];

pub struct ParsedOBJ {
    pub verts: Vec<Vert>,
    pub faces: Vec<Face>,
}

/// Reads in a Wavefront .obj file
///
/// This parser only supports the following:
/// - Geometric vertices, texture coordinates, and normals
/// - **Triangulated** faces (quads/polys will be parsed incorrectly)
/// - `.mtl` diffuse color only
///
/// This parser ignores named objects and groups, treating each file as one object
pub fn parse_obj_file(input_filepath: PathBuf) -> io::Result<ParsedOBJ> {
    let input_file = File::open(input_filepath)?;
    let input = BufReader::new(input_file);

    let mut raw_vert_positions: Vec<[f32; 3]> = Vec::new();
    //let mut raw_vert_normals: Vec<[f32; 3]> = Vec::new();
    //let mut raw_vert_texcoords: Vec<[f32; 2]> = Vec::new();
    //let mut raw_faces: Vec<[usize; 3]> = Vec::new();

    let mut emitted_verts: Vec<Vert> = Vec::new();
    let mut emitted_vert_map: HashMap<VertIndexes, usize> = HashMap::new();
    let mut emitted_faces: Vec<Face> = Vec::new();

    for line in input.lines() {
        let line = line?;
        let mut words = line.split_ascii_whitespace().map(String::from);
        match words.next().unwrap().as_str() {
            "v" => raw_vert_positions.push(parse_vertex(words)),
            "f" => emitted_faces.push(parse_tri(words).map(|v: VertIndexes| {
                if let Some(index) = emitted_vert_map.get(&v) {
                    // this vert already exists
                    return *index;
                } else {
                    // this is a new vert, emit it
                    let index = emitted_verts.len();
                    emitted_vert_map.insert(v, index);
                    let new_vert = Vert {
                        pos: raw_vert_positions[v.vert_index],
                    };
                    emitted_verts.push(new_vert);
                    return index;
                }
            })),

            _ => {}
        }
    }

    Ok(ParsedOBJ {
        verts: emitted_verts,
        faces: emitted_faces,
    })
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

fn parse_tri<I: Iterator<Item = String>>(mut words: I) -> [VertIndexes; 3] {
    let parse_vertex = |s: String| {
        let mut nums = s.splitn(3, '/');
        let vi = nums.next().unwrap().parse::<usize>().unwrap();
        let _vti = nums.next().map(|s| s.parse::<usize>().unwrap());
        let _vni = nums.next().map(|s| s.parse::<usize>().unwrap());
        return VertIndexes { vert_index: vi - 1 };
    };
    return [
        parse_vertex(words.next().unwrap()),
        parse_vertex(words.next().unwrap()),
        parse_vertex(words.next().unwrap()),
    ];
}

//fn parse_mtllib(file)
