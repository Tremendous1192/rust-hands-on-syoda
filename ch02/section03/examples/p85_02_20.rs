fn main() {
    // Create a vector with integer data
    let data = vec![123, 456, 789];

    // Initialize a variable to store the sum
    let mut result = 0;

    // Iterate through the vector and accumulate the sum
    for item in data {
        result += item;
    }

    // Print the sum of the data
    println!("The sum of the data is {}.", result);
}
