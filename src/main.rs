use quick_xml::de::from_str;

use got_label_creator_v3::models::xml::Document;

use std::fs;

fn main() {
    let file: Vec<u16> = fs::read("./tests/Analog.xml")
        .unwrap()
        .chunks(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();
    let xml = String::from_utf16(&file[..]).unwrap().to_string();

    let object: Document = from_str(&xml).unwrap();
    println!("{object:#?}");
}
