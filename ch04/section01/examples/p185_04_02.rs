// ▼ List 4-2
// Main function to demonstrate working with a vector of Option<i32>
fn main() {
    // Create an empty vector to store Option<i32> values
    let mut data = vec![];

    // Iterate over a range and push Some(n) into the vector for each value of n
    for n in 0..5 {
        data.push(Some(n));
    }

    // Call the print_all function to print each item in the vector
    print_all(data);
}

// Function to print all items in a vector of Option<i32>
fn print_all(data: Vec<Option<i32>>) {
    // Iterate over the vector and print each item using println!("{:?}", item)
    for item in data {
        println!("{:?}", item);
    }
}
