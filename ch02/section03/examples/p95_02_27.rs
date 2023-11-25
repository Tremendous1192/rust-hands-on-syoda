fn main() {
    // Create a String with the initial value
    let mut s1 = String::from("Hello,World!");
    println!("s1: {}", s1);

    // Insert a string slice at the specified index
    s1.insert_str(6, " Rust ");
    println!("s1: {}", s1);

    // Insert a character at the specified index
    s1.insert(7, '*');
    println!("s1: {}", s1);

    // Insert another character at a different index
    s1.insert(12, '*');
    println!("s1: {}", s1);

    // Remove the character at the specified index
    s1.remove(5);
    println!("s1: {}", s1);
}
