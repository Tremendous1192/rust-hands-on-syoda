#[derive(Debug)]
// Define a generic struct 'Sample' with fields: name (String) and values (Vec<T>)
struct Sample<T: core::fmt::Debug> {
    name: String,
    values: Vec<T>,
}

impl<T: core::fmt::Debug> Sample<T> {
    // Method to print the values of the 'Sample' instance
    fn print_values(&self) {
        println!("*** {} ***", &self.name);
        for item in &self.values {
            println!("{:?}", item);
        }
    }
}

// Function to create a 'Sample' instance with the given parameters
fn sample<T: core::fmt::Debug>(name: &str, values: Vec<T>) -> Sample<T> {
    Sample {
        name: String::from(name),
        values: values,
    }
}

fn main() {
    // Create a 'Sample' instance 'taro' with an i32 vector
    let taro = sample("Taro", vec![123, 456, 789]);
    // Print the values of 'taro'
    taro.print_values();

    // Create a 'Sample' instance 'hanako' with a string vector
    let hanako = sample("Hanako", vec!["Hello", "Welcome", "Bye!"]);
    // Print the values of 'hanako'
    hanako.print_values();
}
