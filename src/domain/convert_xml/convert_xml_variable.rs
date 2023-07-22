use crate::errors::Errors;

use super::super::models::{inner, input_xml};

pub fn convert_xml_variable(
    xml_var: &input_xml::Variable,
) -> Result<Vec<Box<dyn inner::GetValues>>, Errors> {
    let mut result: Vec<Box<dyn inner::GetValues>> = vec![];
    match xml_var {
        // примитивный тип
        input_xml::Variable {
            type_: input_xml::VariableType { content },
            name,
            address: Some(address),
            ..
        } => {
            result = vec![inner::PrimitiveElement::new(
                name,
                content.clone().try_into()?,
                address,
            )];
            Ok(result)
        }
        // Not Found
        _ => Ok(result),
    }
}
