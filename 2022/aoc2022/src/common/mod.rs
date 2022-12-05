use std::fs;

pub fn read_file(filename: &str) -> String {
    let filedata = fs::read_to_string(&filename)
        .expect("could not read file")
        .trim()
        .to_string();
    filedata
}

pub fn read_file_notrim(filename: &str) -> String {
    let filedata = fs::read_to_string(&filename)
        .expect("could not read file")
        .to_string();
    filedata
}
