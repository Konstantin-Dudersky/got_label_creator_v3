mod application;
mod domain;
mod errors;
mod infrastructure;

use errors::Errors;

pub fn run(input_xml: &str, output_path: &str) -> Result<(), Errors> {
    let file_reader = infrastructure::FileReader::new();
    let xml_reader = infrastructure::XmlReader::new();
    let csv_writer = infrastructure::CsvWriter::new();
    let runner = application::Runner::new(file_reader, xml_reader, csv_writer);
    runner.run(input_xml, output_path)?;
    Ok(())
}
