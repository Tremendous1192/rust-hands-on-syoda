fn main() {
    // Create a String with the initial value
    let s1 = String::from("Hello,Rust World!");

    // Create string slices (references to parts of the original string)
    let s2 = &s1[0..5];
    let s3 = &s1[6..10];
    let s4 = &s1[11..16];

    // Concatenate the slices to create a new String
    let s5 = String::new() + s4 + s3 + s2;

    // Print the result
    println!("{}", s5);
}
