use crate::domain::{
    convert_from_xml::convert_xml_document, convert_to_csv::convert_to_csv,
};
use crate::errors::Errors;

use super::traits;

pub struct Runner {
    file_reader: Box<dyn traits::IFileReader>,
    xml_reader: Box<dyn traits::IXmlReader>,
    csv_writer: Box<dyn traits::ICsvWriter>,
}

impl Runner {
    pub fn new(
        file_reader: Box<dyn traits::IFileReader>,
        xml_reader: Box<dyn traits::IXmlReader>,
        csv_writer: Box<dyn traits::ICsvWriter>,
    ) -> Self {
        Self {
            file_reader,
            xml_reader,
            csv_writer,
        }
    }

    pub fn run(&self, input_xml: &str) -> Result<(), Errors> {
        let xml = self.file_reader.read(input_xml)?;
        let xml = self.xml_reader.read(&xml)?;
        let table_name = xml.instances.configurations.configuration.global_vars
            [0]
        .name
        .clone();
        let values = convert_xml_document(&xml)?;
        let values = convert_to_csv(&table_name, &values);
        let output_file_name = format!("{}.csv", table_name);
        self.csv_writer.write(&output_file_name, &values)?;
        Ok(())
    }
}
