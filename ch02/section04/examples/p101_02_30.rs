fn main() {
    // Call the print_msg function with the argument 100
    print_msg(100);

    // Call the print_msg function with the argument 200
    print_msg(200);

    // Call the print_msg function with the argument 300
    print_msg(300);
}

// Function to print a message, including the sum of numbers up to a specified maximum
fn print_msg(max: i32) {
    // Print a message with the maximum value and the result of the calc function
    println!("The sum of numbers up to {} is {}.", max, calc(max));
}

// Function to calculate the sum of numbers up to a specified maximum
fn calc(max: i32) -> i32 {
    // Initialize a variable to store the sum
    let mut result = 0;

    // Use a for loop to iterate from 0 to (max - 1)
    for n in 0..max {
        // Add the current value of the counter to the sum
        result += n;
    }

    // Return the calculated result
    result
}
