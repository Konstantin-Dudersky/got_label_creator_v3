use crate::errors::Errors;

use super::super::models;
use super::convert_xml_variable::convert_xml_variable;

pub fn convert_xml_document(
    document: &models::input_xml::Document,
) -> Result<Vec<models::inner::PrimitiveElement>, Errors> {
    let global_vars =
        &document.instances.configurations.configuration.global_vars[0]
            .variable;

    let mut result_variables: Vec<models::inner::PrimitiveElement> = vec![];
    for variable in global_vars.iter() {
        result_variables.extend(convert_xml_variable(variable)?);
    }
    Ok(result_variables)
}
