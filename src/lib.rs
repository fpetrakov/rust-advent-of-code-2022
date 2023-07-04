use std::fs::read_to_string;

pub fn get_input_from_file(filename: &str) -> String {
    read_to_string(format!("inputs/{}", filename)).expect("File should be in the inputs folder")
}
