// ▼ List 4-25
use std::fs::File;
use std::io::ErrorKind;
use std::io::{BufRead, BufReader};

fn main() {
    // Attempt to open the file named "data.txt"
    let file = match File::open("data.txt") {
        // If successful, return the file
        Ok(file) => file,

        // If an error occurs, match on the type of error
        Err(error) => match error.kind() {
            // If the file is not found, panic with a specific error message
            ErrorKind::NotFound => panic!("File not found"),

            // If permission is denied, panic with a specific error message
            ErrorKind::PermissionDenied => panic!("Permission denied"),

            // For other errors, panic with a generic error message and print the detailed error
            _ => panic!("Failed to open the file: {:?}", error),
        },
    };

    // Create a buffered reader to efficiently read lines from the file
    let reader = BufReader::new(file);

    // Iterate over the lines in the file using the buffered reader and print each line
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
