// ▼ List 4-29
use std::fs;

fn main() {
    // Read the contents of the current directory and unwrap the result
    let paths = fs::read_dir("./").unwrap();

    // Iterate over the entries in the directory
    for path in paths {
        // Unwrap each directory entry
        let entry = path.unwrap();

        // Get the file type of the entry and unwrap the result
        let ftype = entry.file_type().unwrap();

        // Check the file type and print the corresponding message
        if ftype.is_file() {
            println!("{:?} file", entry.path())
        } else if ftype.is_dir() {
            println!("{:?} dir", entry.path())
        } else if ftype.is_symlink() {
            println!("{:?} link", entry.path())
        } else {
            println!("{:?}", entry.path())
        }
    }
}
