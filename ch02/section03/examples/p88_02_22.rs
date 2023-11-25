use std::collections::HashMap;

fn main() {
    // Create a new HashMap with String keys and i32 values
    let mut map = HashMap::new();

    // Insert key-value pairs into the HashMap
    map.insert(String::from("first"), 123);
    map.insert(String::from("second"), 456);
    map.insert(String::from("third"), 789);

    // Remove the key "second" and its associated value from the HashMap
    map.remove("second");

    // Print the final state of the HashMap
    println!("{:?}", map);
}
