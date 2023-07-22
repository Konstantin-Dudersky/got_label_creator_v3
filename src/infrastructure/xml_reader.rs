//! Конвертация XML в структуру

use quick_xml::de::{from_str, DeError};

use crate::application::IXmlReader;
use crate::domain::models::input_xml::Document;
use crate::errors::Errors;

pub struct XmlReader {}

impl XmlReader {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl IXmlReader for XmlReader {
    fn read(&self, file: &str) -> Result<Document, Errors> {
        let document: Document = from_str(&file)?;
        Ok(document)
    }
}

impl From<DeError> for Errors {
    fn from(value: DeError) -> Self {
        let msg = format!("Ошибка десериализации документа: {}", value);
        Errors::DeserializationError(msg)
    }
}
