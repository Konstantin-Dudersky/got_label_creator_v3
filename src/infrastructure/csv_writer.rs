//! Запись файла csv
//! Данные на входе - вектор векторов строк, Vec<Vec<&str>>.
//! Вложенные вектора должны быть одикановой длины

use std::fs::File;

use csv::Writer;

use crate::application::ICsvWriter;
use crate::errors::Errors;

pub struct CsvWriter {}

impl CsvWriter {
    pub fn new() -> Self {
        Self {}
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
        data: Vec<Vec<&str>>,
    ) -> Result<(), Errors> {
        let mut wtr = self.create_writer(file_name)?;
        for line in data {
            self.write_line(&mut wtr, &line)?;
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

        let data = vec![vec!["1", "2", "3"], vec!["4", "5", "6"]];

        let writer = CsvWriter::new();

        writer.write("./temp/csv_writer_test1.csv", data).unwrap();
    }
}
