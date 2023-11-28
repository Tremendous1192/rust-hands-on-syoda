// ▼ List 4-28
use std::fs;

fn main() {
    // Read the contents of the current directory and unwrap the result
    let paths = fs::read_dir("./").unwrap();

    // Iterate over the entries in the directory
    for path in paths {
        // Unwrap each directory entry
        let entry = path.unwrap();

        // Print the path of the directory entry as a string
        println!("{:?}", entry.path().to_str());
    }
}
