use crate::errors::Errors;

use super::super::models::{inner, input_xml};
use super::convert_xml_variable::convert_xml_variable;

pub fn convert_xml_document(
    document: &input_xml::Document,
) -> Result<Vec<Box<dyn inner::GetValues>>, Errors> {
    let global_vars =
        &document.instances.configurations.configuration.global_vars[0]
            .variable;

    let mut result_variables: Vec<Box<dyn inner::GetValues>> = vec![];
    for variable in global_vars.iter() {
        result_variables.extend(convert_xml_variable(variable)?);
    }
    Ok(result_variables)
}
