fn main() {
    // Create an empty String
    let mut s1 = String::new();
    println!("s1: {}", s1);

    // Append a string slice to the existing String
    s1.push_str("Hello");
    println!("s1: {}", s1);

    // Append another string slice to the existing String
    s1.push_str("World!");
    println!("s1: {}", s1);
}
