use std::fs;

use crate::errors::Errors;

pub fn read_file(filename: &str) -> Result<String, Errors> {
    let file: Vec<u16> = fs::read(filename)?
        .chunks(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();
    let str = String::from_utf16(&file)?.to_string();
    Ok(str)
}
