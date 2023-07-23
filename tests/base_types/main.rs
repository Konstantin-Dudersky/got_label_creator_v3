use std::fs;

use got_label_creator_v3::run;

#[test]
fn main() {
    let base_path = "./tests/base_types";
    let input_xml = format!("{}/base_types.xml", base_path);
    let csv_reference = format!("{}/base_types_reference.csv", base_path);
    let csv_test = format!("{}/base_types.csv", base_path);

    run(&input_xml, base_path).unwrap();

    let reference = fs::read(csv_reference).unwrap();
    let reference = String::from_utf8_lossy(&reference);
    let test = fs::read(csv_test).unwrap();
    let test = String::from_utf8_lossy(&test);

    assert_eq!(reference, test);

    fs::remove_file("./tests/base_types/base_types.csv").unwrap();
}
