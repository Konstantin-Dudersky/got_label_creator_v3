use got_label_creator_v3::application::Runner;
use got_label_creator_v3::infrastructure::{CsvWriter, FileReader, XmlReader};

fn main() {
    let filename = "./tests/base_types.xml";

    let file_reader = FileReader::new();
    let xml_reader = XmlReader::new();
    let csv_writer = CsvWriter::new();
    let runner = Runner::new(file_reader, xml_reader, csv_writer);

    runner.run(filename).unwrap();
}
