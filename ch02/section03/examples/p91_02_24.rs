use std::collections::HashMap;

fn main() {
    // Create a new HashMap with String keys and i32 values
    let mut map = HashMap::new();

    // Insert key-value pairs into the HashMap
    map.insert(String::from("first"), 123);
    map.insert(String::from("second"), 456);
    map.insert(String::from("third"), 789);

    // Initialize a variable to store the sum of values
    let mut result = 0;

    // Iterate over key-value pairs in the HashMap
    for (key, value) in map {
        // Print each key-value pair
        println!("{}: {}.", key, value);

        // Accumulate the values to calculate the total sum
        result += value;
    }

    // Print the total sum of values
    println!("total: {}.", result);
}
