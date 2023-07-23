use crate::errors::Errors;

use super::input_xml;

#[derive(Clone, Debug)]
pub enum PrimitiveType {
    Bool,
    Dint,
    Dword,
    Int,
    Real,
    Word,
}

impl Into<String> for PrimitiveType {
    fn into(self) -> String {
        match self {
            PrimitiveType::Bool => "Bit".to_string(),
            PrimitiveType::Dint => "Signed BIN32".to_string(),
            PrimitiveType::Dword => "Unsigned BIN32".to_string(),
            PrimitiveType::Int => "Signed BIN16".to_string(),
            PrimitiveType::Real => "Real(32bit)".to_string(),
            PrimitiveType::Word => "Unsigned BIN16".to_string(),
        }
    }
}

impl TryFrom<input_xml::VariableTypes> for PrimitiveType {
    type Error = Errors;

    fn try_from(value: input_xml::VariableTypes) -> Result<Self, Self::Error> {
        match value {
            input_xml::VariableTypes::Array(_)
            | input_xml::VariableTypes::Derived => {
                let description = format!("{:?}", value);
                let error = Errors::UnknownDataTypeInXml(description);
                Err(error)
            }
            input_xml::VariableTypes::Bool => Ok(PrimitiveType::Bool),
            input_xml::VariableTypes::Dint => Ok(PrimitiveType::Dint),
            input_xml::VariableTypes::Dword => Ok(PrimitiveType::Dword),
            input_xml::VariableTypes::Int => Ok(PrimitiveType::Int),
            input_xml::VariableTypes::Real => Ok(PrimitiveType::Real),
            input_xml::VariableTypes::Time => Ok(PrimitiveType::Dword),
            input_xml::VariableTypes::Word => Ok(PrimitiveType::Word),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrimitiveElement {
    pub name: String,
    pub data_type: PrimitiveType,
    pub device: String,
}

impl PrimitiveElement {
    pub fn new(name: &str, data_type: PrimitiveType, device: &str) -> Self {
        Self {
            name: name.to_string(),
            data_type: data_type,
            device: device.to_string(),
        }
        .into()
    }
}
