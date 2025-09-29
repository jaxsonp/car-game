use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use car_game_csv::CSVInstance;
use car_game_obj::{OBJMaterial, OBJMesh};
use nalgebra::{Scale3, Translation3, UnitQuaternion};
use workspace_root::get_workspace_root;

/// This build script re-parses obj models into `RawMesh`es
///
/// To be specific, for every `xyz.obj` file in the models directory, this script produces an `xyz.obj.rs` file in
/// `OUT_DIR`, containing a value of the following type: `&[RawMesh]`, with one `RawMesh` per material in the file
fn main() {
    let assets_dir = get_workspace_root().join("assets");

    println!("cargo::rerun-if-changed={}", assets_dir.display());

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    for file in std::fs::read_dir(assets_dir).unwrap() {
        if file.is_err() {
            continue;
        }
        let file_path = file.unwrap().path();
        if !file_path.is_file() {
            continue;
        }

        let ext = file_path.extension();
        if ext.is_some_and(|ext| ext.eq_ignore_ascii_case("obj")) {
            // obj file
            let output_path = out_dir
                .join(file_path.file_name().unwrap())
                .with_extension("obj.rs");
            println!(
                "Preloading OBJ file \'{}\' (output at: \'{}\')",
                file_path.display(),
                output_path.display(),
            );
            match car_game_obj::parse_obj_mesh(file_path.clone()) {
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
        } else if ext.is_some_and(|ext| ext.eq_ignore_ascii_case("csv")) {
            // csv instance file
            let output_path = out_dir
                .join(file_path.file_name().unwrap())
                .with_extension("csv.rs");
            println!(
                "Preloading CSV file \'{}\' (output at: \'{}\')",
                file_path.display(),
                output_path.display(),
            );
            match car_game_csv::parse_instances_csv(file_path.clone()) {
                Ok(instances) => {
                    emit_parsed_instances(instances, output_path)
                        .expect("Error while writing instance data");
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
            output.write(
                format!("\t\t\t{}u32, {}u32, {}u32,\n", face[0], face[1], face[2]).as_bytes(),
            )?;
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

fn emit_parsed_instances(instances: Vec<CSVInstance>, file_path: PathBuf) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut output = BufWriter::new(file);

    output.write(b"// Generated via build script\n")?;
    output.write(b"&[\n")?;
    for instance in instances.iter() {
        let matrix = {
            let translation = Translation3::from(instance.position);
            let rotation = UnitQuaternion::from_euler_angles(
                instance.rotation[0],
                instance.rotation[1],
                instance.rotation[2],
            );
            let scale = Scale3::from(instance.scale);
            translation.to_homogeneous() * rotation.to_homogeneous() * scale.to_homogeneous()
        };
        output.write(b"    [")?;
        for col in matrix.data.0 {
            output.write(
                format!("[{}f32,{}f32,{}f32,{}f32],", col[0], col[1], col[2], col[3]).as_bytes(),
            )?;
        }
        output.write(b"],\n")?;
    }
    output.write(b"]")?;
    Ok(())
}
