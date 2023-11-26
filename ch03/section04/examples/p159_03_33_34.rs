// Import the 'Debug' trait for automatic debugging support
#[derive(Debug)]
// Define a generic struct 'Sample' with fields: name (String) and value (T)
struct Sample<T> {
    name: String,
    value: T,
}

// Function to create a 'Sample' instance with the given parameters
fn sample<T>(name: &str, value: T) -> Sample<T> {
    Sample {
        name: String::from(name),
        value: value,
    }
}

fn main() {
    // Create a 'Sample' instance 'taro' with a String value
    let taro = sample("Taro", "this is message.");
    // Print the debug representation of 'taro'
    println!("{:?}", taro);

    // Create a 'Sample' instance 'hanako' with an i32 value
    let hanako = sample("Hanako", 1234);
    // Print the debug representation of 'hanako'
    println!("{:?}", hanako);
}
