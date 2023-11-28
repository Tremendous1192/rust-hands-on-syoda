// ▼ List 4-27
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Text data to be appended to the file
    let str_data = "This is sample!\n"; //☆

    // Open the file "append.txt" with options to create and append, then unwrap the result
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("append.txt")
        .unwrap();

    // Write the content of the string to the end of the file and unwrap the result
    file.write_all(str_data.as_bytes()).unwrap();
}
