use got_label_creator_v3::application::Runner;
use got_label_creator_v3::infrastructure::{FileReader, XmlReader};

fn main() {
    let filename = "./tests/base_types.xml";

    let file_reader = FileReader::new();
    let xml_reader = XmlReader::new();
    let runner = Runner::new(file_reader, xml_reader);

    runner.run(filename).unwrap();
}
