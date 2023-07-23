use crate::errors::Errors;

use super::super::models;

pub fn convert_xml_variable(
    xml_var: &models::input_xml::Variable,
) -> Result<Vec<models::inner::PrimitiveElement>, Errors> {
    let mut result: Vec<models::inner::PrimitiveElement> = vec![];
    match xml_var {
        // примитивный тип
        models::input_xml::Variable {
            type_: models::input_xml::VariableType { content },
            name,
            address: Some(address),
            ..
        } => {
            result = vec![models::inner::PrimitiveElement::new(
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
