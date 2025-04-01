use std::fs::OpenOptions;
use std::io::Write;

pub fn append_to_csv(path:&str, line:String) {

    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Failed to open file")
        .write_all(line.as_bytes())
        .expect("Failed to append to file");                

}