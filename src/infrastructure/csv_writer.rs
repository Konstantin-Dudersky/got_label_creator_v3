//! Запись файла csv
//! Данные на входе - вектор векторов строк, Vec<Vec<&str>>.
//! Вложенные вектора должны быть одикановой длины

use std::fs::File;

use csv::Writer;

use crate::application::ICsvWriter;
use crate::errors::Errors;

pub struct CsvWriter {}

impl CsvWriter {
    pub fn new() -> Box<Self> {
        Self {}.into()
    }

    fn create_writer(&self, file_name: &str) -> Result<Writer<File>, Errors> {
        match Writer::from_path(file_name) {
            Ok(writer) => Ok(writer),
            Err(error) => {
                let error_msg = error.to_string();
                let error = Errors::CsvFileCreationError(error_msg);
                return Err(error);
            }
        }
    }

    fn write_line(
        &self,
        writer: &mut Writer<File>,
        line: &Vec<&str>,
    ) -> Result<(), Errors> {
        match writer.write_record(line) {
            Ok(_) => Ok(()),
            Err(error) => {
                let error_msg = error.to_string();
                let error = Errors::CsvWriteLineError(error_msg);
                return Err(error);
            }
        }
    }
}

impl ICsvWriter for CsvWriter {
    fn write(
        &self,
        file_name: &str,
        data: &Vec<Vec<String>>,
    ) -> Result<(), Errors> {
        let mut wtr = self.create_writer(file_name)?;
        for line in data {
            let line_str: Vec<&str> = line.iter().map(String::as_str).collect();
            self.write_line(&mut wtr, &line_str)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test1() {
        fs::create_dir_all("./temp").unwrap();

        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];

        let writer = CsvWriter::new();

        writer.write("./temp/csv_writer_test1.csv", &data).unwrap();
    }
}
