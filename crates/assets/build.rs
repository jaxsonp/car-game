use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use obj::{OBJMaterial, OBJMesh};
use workspace_root::get_workspace_root;

/// This build script re-parses obj models into `RawMesh`es
///
/// To be specific, for every `xyz.obj` file in the models directory, this script produces an `xyz.obj.rs` file in
/// `OUT_DIR`, containing a value of the following type: `&[RawMesh]`, with one `RawMesh` per material in the file
fn main() {
    let assets_dir = get_workspace_root().join("assets");
    let fonts_dir = assets_dir.join("fonts");
    let meshes_dir = assets_dir.join("meshes");

    println!("cargo::rustc-env=FONTS_DIR={}", fonts_dir.display());

    println!("cargo::rerun-if-changed={}", assets_dir.display());

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    for file in std::fs::read_dir(meshes_dir).unwrap() {
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
            "Preloading OBJ file \'{}\' (output at: \'{}\')",
            file_path.display(),
            output_path.display(),
        );

        match obj::parse_obj_file(file_path.clone()) {
            Ok(meshes) => {
                emit_parsed_obj(meshes, output_path).expect("Error while writing parsed OBJ");
            }
            Err(e) => {
                println!(
                    "cargo::error=Error while parsing \'{}\': {}",
                    file_path.to_str().unwrap(),
                    e
                );
            }
        }
    }
}

fn emit_parsed_obj(meshes: Vec<(OBJMaterial, OBJMesh)>, file_path: PathBuf) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut output = BufWriter::new(file);

    output.write(b"// Baked mesh, generated via build script\n")?;

    output.write(b"&[\n")?;
    for (material, mesh) in meshes {
        output.write(b"\tRawMesh {\n")?;
        output.write(b"\t\tverts: &[\n")?;
        for vert in mesh.verts {
            output.write(
                format!(
                    "\t\t\tRawVertex {{ pos: [{}f32, {}f32, {}f32], normal: [{}f32, {}f32, {}f32] }},\n",
                    vert.pos[0], vert.pos[1], vert.pos[2],
                    vert.normal[0], vert.normal[1], vert.normal[2]
                )
                .as_bytes(),
            )?;
        }
        output.write(b"\t\t],\n")?;
        output.write(b"\t\tindices: &[\n")?;
        for face in mesh.faces {
            output.write(format!("\t\t\t{}, {}, {},\n", face[0], face[1], face[2]).as_bytes())?;
        }
        output.write(b"\t\t],\n")?;
        output.write(
            format!(
                "\t\tmaterial: RawMaterial {{ color: [{}f32, {}f32, {}f32] }},\n",
                material.diffuse_color[0], material.diffuse_color[1], material.diffuse_color[2]
            )
            .as_bytes(),
        )?;
        output.write(b"\t},\n")?;
    }
    output.write(b"]")?;
    Ok(())
}
