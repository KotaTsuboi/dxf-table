use std::error::Error;
use std::io::Read;
use std::{fs, io::BufReader};

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TomlInput {
    pub cells: Vec<Vec<String>>,
    pub cell_widths: Vec<f64>,
    pub cell_height: f64,
    pub text_height: f64,
    pub relative_x_scale_factor: f64,
    pub primary_font_file_name: String,
    pub big_font_file_name: String,
}

fn read_file(path: &str) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(BufReader::new)
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn read_input(file_path: &str) -> Result<TomlInput, Box<dyn Error>> {
    let s = read_file(file_path).expect("failed to read file");

    let toml: Result<TomlInput, toml::de::Error> = toml::from_str(&s);

    let toml = toml.expect("failed to parse toml");

    Ok(toml)
}
