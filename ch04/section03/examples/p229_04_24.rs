// ▼ List 4-24
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file named "data.txt"; panic if the file does not exist or encounters an error
    let file = File::open("data.txt").unwrap();

    // Create a buffered reader to efficiently read lines from the file
    let reader = BufReader::new(file);

    // Initialize a counter for line numbers
    let mut count = 0;

    // Iterate over the lines in the file using the buffered reader
    for line in reader.lines() {
        // Increment the line number counter
        count += 1;

        // Unwrap the line, panic if an error occurs
        let txt = line.unwrap();

        // Print the line number and the text of the line
        println!("{}: {}", count, txt);
    }
}
