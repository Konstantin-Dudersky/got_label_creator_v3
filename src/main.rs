use quick_xml::de::from_str;

use got_label_creator_v3::infrastructure::file_reader::read_file;
use got_label_creator_v3::models::xml::{Document, Variable, VariableType, VariableTypes};

fn main() {
    let xml = read_file("./tests/base_types.xml").unwrap();

    let object: Document = from_str(&xml).unwrap();

    let global_vars = &object.instances.configurations.configuration.global_vars[0];
    let table_name = global_vars.name.clone();
    let var1 = &global_vars.variable[0];

    match var1 {
        Variable {
            type_: VariableType {
                content: VariableTypes::Bool,
            },
            name,
            ..
        } => println!("found, name: {}", name),
        _ => println!("not found"),
    }
    // println!("{:#?}", var1);
}
