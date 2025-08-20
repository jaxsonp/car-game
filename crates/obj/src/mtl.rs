use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use crate::OBJMaterial;

/// reads an mtl file, creating new materials and updating the material table and list accordingly
pub fn parse_mtl_file(
    input_filepath: PathBuf,
    mat_list: &mut Vec<OBJMaterial>,
    mat_table: &mut HashMap<String, usize>,
) -> io::Result<()> {
    let input_file = File::open(input_filepath)?;
    let input = BufReader::new(input_file);

    let mut current_material: Option<&mut OBJMaterial> = None; //OBJMaterial::default();

    for line in input.lines() {
        let line = line?;
        let mut words = line.split_ascii_whitespace().map(String::from);

        let first_word = words.next();
        if first_word.is_none() {
            continue;
        }
        match first_word.unwrap().as_str() {
            "newmtl" => {
                let name = words.next().unwrap();
                mat_list.push(OBJMaterial::new(name.clone()));
                mat_table.insert(name, mat_list.len() - 1);
                current_material = mat_list.last_mut();
            }
            "Kd" => {
                // diffuse color
                if let Some(current_material) = &mut current_material {
                    let r = words.next().unwrap().parse::<f32>().unwrap();
                    let g = words.next().unwrap().parse::<f32>().unwrap();
                    let b = words.next().unwrap().parse::<f32>().unwrap();
                    current_material.diffuse_color = [r, g, b];
                }
            }
            _ => {}
        }
    }

    Ok(())
}
