// ▼ List 4-26
use std::fs::File;
use std::io::Write;

fn main() {
    // Sample data to be written to the file
    let data = [
        "Hello world!",
        "これはサンプルのデータです。",
        "テストテスト！",
    ];

    // Join the data into a single string with newline separators
    let str_data = data.join("\n");

    // Create or overwrite the file named "backup.txt" and unwrap the result
    let mut file = File::create("backup.txt").unwrap();

    // Write the entire content of the string to the file and unwrap the result
    file.write_all(str_data.as_bytes()).unwrap();
}
