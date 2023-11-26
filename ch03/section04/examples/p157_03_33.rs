// Import the 'Debug' trait for automatic debugging support
#[derive(Debug)]
// Define a generic struct 'Sample' with fields: name (String) and value (T)
struct Sample<T> {
    name: String,
    value: T,
}

fn main() {
    // Create a 'Sample' instance 'taro' with a String value
    let taro = Sample {
        name: String::from("Taro"),
        value: String::from("this is message."),
    };
    // Print the debug representation of 'taro'
    println!("{:?}", taro);

    // Create a 'Sample' instance 'hanako' with an i32 value
    let hanako = Sample {
        name: String::from("Hanako"),
        value: 1234,
    };
    // Print the debug representation of 'hanako'
    println!("{:?}", hanako);
}
