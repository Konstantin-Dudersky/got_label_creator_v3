use crate::domain::convert_xml::convert_xml_document;
use crate::errors::Errors;

use super::traits;

pub struct Runner {
    file_reader: Box<dyn traits::IFileReader>,
    xml_reader: Box<dyn traits::IXmlReader>,
}

impl Runner {
    pub fn new(
        file_reader: Box<dyn traits::IFileReader>,
        xml_reader: Box<dyn traits::IXmlReader>,
    ) -> Self {
        Self {
            file_reader,
            xml_reader,
        }
    }

    pub fn run(&self, file_name: &str) -> Result<(), Errors> {
        let xml = self.file_reader.read(file_name)?;
        let xml = self.xml_reader.read(&xml)?;

        let result = convert_xml_document(&xml)?;

        for r in &result {
            println!("{:?}", r.get_values());
        }
        Ok(())
    }
}
