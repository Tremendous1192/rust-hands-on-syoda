use rand::Rng;

// ▼ List 4-3
// Function to generate a random Option<i32>
fn random() -> Option<i32> {
    // Generate a random number between 0 and 9
    let n = rand::thread_rng().gen_range(0..10);
    // Return None if the generated number is 0, otherwise return Some(n)
    match n {
        0 => None,
        _ => Some(n),
    }
}

// ▼ List 4-4
fn main() {
    // Create an empty vector to store random Option<i32> values
    let mut data = vec![];

    // Generate and push random Option<i32> values into the vector for 10 iterations
    for _ in 0..10 {
        data.push(random());
    }

    // Call the print_all function to print each item in the vector
    print_all(data);
}

// Function to print all items in a vector of Option<i32>
fn print_all(data: Vec<Option<i32>>) {
    // Iterate over the vector and call the print function for each item
    for item in data {
        // Call the print function and handle the Result returned
        match print(item) {
            Ok(status) => println!("{}", status),
            Err(error) => println!("Error: {}", error),
        };
    }
}

// ▼ List 4-7
// Function to print an individual Option<i32> and return a Result
fn print(item: Option<i32>) -> Result<String, String> {
    // Match on the Option and return an Err with a custom message if None, otherwise print the value
    match item {
        None => Err(String::from("ERROR IS OCCURRED.")),
        Some(n) => {
            println!("No, {}.", n);
            Ok(String::from("OK"))
        }
    }
}
