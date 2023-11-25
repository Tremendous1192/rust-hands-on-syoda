use std::collections::HashMap;

fn main() {
    // Create a new HashMap with String keys and i32 values
    let mut map = HashMap::new();

    // Insert key-value pairs into the HashMap
    map.insert(String::from("first"), 123);

    // Access the value associated with the key "first" and use it to calculate a new value for the key "second"
    map.insert(String::from("second"), map["first"] * 2);

    // Access the values associated with the keys "first" and "second" to calculate a new value for the key "third"
    map.insert(
        String::from("third"),
        map.get("first").unwrap() + map.get("second").unwrap(),
    );

    // Print the final state of the HashMap
    println!("{:?}.", map);
}
