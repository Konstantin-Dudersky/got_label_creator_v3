use std::io;
use std::string;

#[derive(Debug)]
pub enum Errors {
    ReadFile(io::Error),
    FromUtf16Error(string::FromUtf16Error),
    UnknownDataTypeInXml(String),
    DeserializationError(String),
}

impl From<io::Error> for Errors {
    fn from(value: io::Error) -> Self {
        Errors::ReadFile(value)
    }
}

impl From<string::FromUtf16Error> for Errors {
    fn from(value: string::FromUtf16Error) -> Self {
        Errors::FromUtf16Error(value)
    }
}
