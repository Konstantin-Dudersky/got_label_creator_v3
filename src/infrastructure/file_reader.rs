//! Чтение файла и конвертация из UTF-16 в UTF-8

use std::fs;

use crate::application::IFileReader;
use crate::errors::Errors;

pub struct FileReader {}

impl FileReader {
    pub fn new() -> Box<Self> {
        let obj = Self {};
        Box::new(obj)
    }
}

impl IFileReader for FileReader {
    fn read(&self, file_name: &str) -> Result<String, Errors> {
        let file: Vec<u16> = fs::read(file_name)?
            .chunks(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect();
        let str = String::from_utf16(&file)?.to_string();
        Ok(str)
    }
}
