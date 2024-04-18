use anyhow::Result;
use serde_derive::Deserialize;
use std::io::Read;
use std::{fs, io::BufReader};

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

fn read_file(path: &str) -> Result<String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path).map(BufReader::new)?;

    fr.read_to_string(&mut file_content)?;

    Ok(file_content)
}

pub fn read_input(file_path: &str) -> Result<TomlInput> {
    let s = read_file(file_path).expect("failed to read file");

    let toml: Result<TomlInput, toml::de::Error> = toml::from_str(&s);

    let toml = toml.expect("failed to parse toml");

    Ok(toml)
}
