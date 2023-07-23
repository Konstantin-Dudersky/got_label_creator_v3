use super::super::models;

const LINE_LEN: usize = 20;

pub fn convert_to_csv(
    table_name: &str,
    input: &Vec<models::inner::PrimitiveElement>,
) -> Vec<Vec<String>> {
    let mut document: Vec<Vec<String>> = vec![];
    document.push(create_line_1(table_name));
    document.push(create_line_2());
    document.push(create_line_3());
    document.push(create_line_4());
    for input_line in input {
        let mut line = create_empty_line();
        line[1] = input_line.name.clone();
        line[2] = input_line.data_type.clone().into();
        line[3] = input_line.device.clone();
        document.push(line)
    }
    document
}

fn create_empty_line() -> Vec<String> {
    vec!["".to_string(); LINE_LEN]
}

fn create_line_1(table_name: &str) -> Vec<String> {
    let mut line = create_empty_line();
    line[1] = table_name.to_string();
    line
}

fn create_line_2() -> Vec<String> {
    create_empty_line()
}

fn create_line_3() -> Vec<String> {
    create_empty_line()
}

fn create_line_4() -> Vec<String> {
    let mut record = create_empty_line();
    record[1] = String::from("Label Name");
    record[2] = String::from("Data Type");
    record[3] = String::from("Assign (Device)");
    record[4] = String::from("Comment");
    record[5] = String::from("Comment2");
    record[6] = String::from("Comment3");
    record[7] = String::from("Comment4");
    record[8] = String::from("Comment5");
    record[9] = String::from("Comment6");
    record[10] = String::from("Comment7");
    record[11] = String::from("Comment8");
    record[12] = String::from("Comment9");
    record[13] = String::from("Comment10");
    record[14] = String::from("Comment11");
    record[15] = String::from("Comment12");
    record[16] = String::from("Comment13");
    record[17] = String::from("Comment14");
    record[18] = String::from("Comment15");
    record[19] = String::from("Comment16");
    record
}
