// ▼ List 4-30
use std::fs;

fn main() {
    // Create a directory named "backup" in the current directory
    let _ = fs::create_dir("./backup");

    // Read the contents of the current directory and unwrap the result
    let entries = fs::read_dir("./").unwrap();

    // Iterate over the entries in the directory
    for path in entries {
        // Unwrap each directory entry
        let entry = path.unwrap();

        // Check if the entry is a file
        if entry.file_type().unwrap().is_file() {
            // Get the file name
            let file_name = entry.file_name();

            // Formulate the source and destination file paths
            let from_name = format!("./{}", file_name.to_string_lossy());
            let to_name = format!("./backup/_{}", file_name.to_string_lossy());

            // Copy the file from source to destination
            let _ = fs::copy(&from_name, &to_name);

            // Print a message indicating the backup
            println!("backup: {:?} → {}", from_name, to_name);
        } else {
            // Print a message for entries that are not files
            println!("not copied.({:?})", entry.file_name());
        }
    }
}
