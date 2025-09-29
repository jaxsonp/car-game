use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

pub struct CSVInstance {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

/// Parses a CSV file containing data about model instances
///
/// Expects the following row structure with no header:
/// | x pos | y pos | z pos | x rot | y rot | z rot | x scale | y scale | z scale |
pub fn parse_instances_csv(input_filepath: PathBuf) -> io::Result<Vec<CSVInstance>> {
    let file = File::open(input_filepath)?;
    let csv_reader = io::BufReader::new(file);

    Ok(csv_reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split(',');

            let row: [f32; 9] = [0; 9].map(|_| {
                let s = split.next().unwrap();
                s.parse::<f32>()
                    .expect(format!("Failed to parse float: {}", s).as_str())
            });
            CSVInstance {
                position: [row[0], row[1], row[2]],
                rotation: [row[3], row[4], row[5]],
                scale: [row[6], row[7], row[8]],
            }
        })
        .collect())
}
