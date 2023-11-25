fn main() {
    // Create an empty vector to store integers
    let mut data = Vec::new();

    // Add integers to the vector
    data.push(123);
    data.push(456);
    data.push(789);

    // Access and print the values in the vector
    println!(
        "0: {}, 1: {}, 2: {}.",
        data[0],
        data[1],
        data.get(2).unwrap()
    );
}
