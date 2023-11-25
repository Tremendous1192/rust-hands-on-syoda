fn main() {
    // Set the maximum value for calculation to 100
    let max = 100;

    // Call the 'calc' function with the maximum value and store the result
    let res = calc(max);

    // Define a closure named 'print_msg' to print the message
    let print_msg = || {
        // Print the message with the maximum value and the result
        println!("The sum of numbers up to {} is {}.", max, res);
    };

    // Call the 'print_msg' closure to display the result
    print_msg();

    // Change the maximum value for calculation to 200
    let max = 200;

    // Call the 'calc' function with the updated maximum value and store the result
    let res = calc(max);

    // Redefine the 'print_msg' closure for the updated calculation
    let print_msg = || {
        // Print a message indicating the range and the total
        println!("Sum of numbers from 0 to {} is {}.", max, res);
    };

    // Call the 'print_msg' closure to display the updated result
    print_msg();
}

// Function to calculate the sum of numbers up to a given maximum value
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
