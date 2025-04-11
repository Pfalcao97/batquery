use std::fs::OpenOptions;
use std::io::Write;

pub fn append_to_csv(path: &str, line: String) {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Failed to open file")
        .write_all(line.as_bytes())
        .expect("Failed to append to file");
}

#[macro_export]
macro_rules! elements_to_csv_line {

    ($first:expr) => {
        format!("{}", $first)
    };

    ($first:expr, $($rest:expr),+) => {
        format!("{}{}", $first, {
            let mut result = String::new();
            $(
                result.push_str(&format!(",{}", $rest));
            )*
            result
        })
    };
}
