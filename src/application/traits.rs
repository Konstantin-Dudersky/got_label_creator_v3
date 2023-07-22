use crate::domain::models::input_xml::Document;
use crate::errors::Errors;

pub trait IFileReader {
    fn read(&self, file_name: &str) -> Result<String, Errors>;
}
pub trait IXmlReader {
    fn read(&self, file: &str) -> Result<Document, Errors>;
}