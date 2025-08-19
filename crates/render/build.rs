use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use obj::ParsedOBJ;

fn main() {
    let assets_dir = "../../assets";
    println!("cargo::rerun-if-changed={}", assets_dir);

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    for file in std::fs::read_dir(assets_dir).unwrap() {
        if file.is_err() {
            continue;
        }
        let file_path = file.unwrap().path();

        if !file_path.is_file()
            || !file_path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("obj"))
        {
            continue; // not an obj file
        }

        let output_path = out_dir
            .join(file_path.file_name().unwrap())
            .with_extension("obj.rs");

        println!(
            "cargo::warning=Parsing OBJ file \'{}\'\t(output: \'{}\')",
            file_path.to_str().unwrap(),
            output_path.to_str().unwrap()
        );
        match obj::parse_obj_file(file_path.clone()) {
            Ok(mesh) => {
                emit_parsed_obj(mesh, output_path).expect("Error while writing parsed OBJ");
            }
            Err(e) => {
                println!(
                    "cargo::error=Error while parsing {}\n{}",
                    file_path.to_str().unwrap(),
                    e
                );
            }
        }
    }
}

fn emit_parsed_obj(mesh: ParsedOBJ, file_path: PathBuf) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut output = BufWriter::new(file);

    output.write(b"// Baked mesh, generated via build script\n")?;

    output.write(b"Mesh {\n")?;
    output.write(b"\tverts: &[\n")?;
    for vert in mesh.verts {
        output.write(
            format!(
                "\t\tVertex {{ pos: [{}f32, {}f32, {}f32] }},\n",
                vert.pos[0], vert.pos[1], vert.pos[2]
            )
            .as_bytes(),
        )?;
    }
    output.write(b"\t],\n")?;
    output.write(b"\tvert_normals: false,\n")?;
    output.write(b"\tvert_texcoords: false,\n")?;
    output.write(b"\tindices: &[\n")?;
    for face in mesh.faces {
        output.write(format!("\t\t{}, {}, {},\n", face[0], face[1], face[2]).as_bytes())?;
    }
    output.write(b"\t],\n")?;
    output.write(b"\tcolor: [0.9, 0.2, 0.2],\n")?;
    output.write(b"}")?;
    Ok(())
}

/*
Mesh {
    pub verts: &'static [Vertex],
    pub vert_normals: bool,
    pub vert_texcoords: bool,
    pub indices: &'static [u16],
    pub color: [f32; 3],
}

Vertex {
    pub pos: [f32; 3],
}
*/
