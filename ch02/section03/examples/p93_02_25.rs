fn main() {
    // Create an empty String
    let s1 = String::new();
    println!("s1: {}", s1);

    // Create a String from a string literal
    let s2 = String::from("Hello");
    println!("s2: {}", s2);

    // Create a string slice (string literal)
    let s3 = "World";
    println!("s3: {}", s3);

    // Concatenate strings using the `+` operator
    let s4 = s1 + &s2 + &s3;
    println!("s4 = s1 + &s2 + &s3: {}", s4);
}
