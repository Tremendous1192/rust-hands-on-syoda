fn main() {
    // Define a closure named 'calc' that takes a parameter 'max'
    let calc = |max| {
        // Initialize a variable to store the sum
        let mut result = 0;

        // Use a for loop to iterate from 0 to (max - 1)
        for n in 0..max {
            // Add the current value of the counter to the sum
            result += n;
        }

        // Return the calculated result
        result
    };

    // Define a closure named 'print_msg' that takes a parameter 'max'
    let print_msg = |max| {
        // Print a message with the maximum value and the result of the 'calc' closure
        println!("The sum of numbers up to {} is {}.", max, calc(max));
    };

    // Call the 'print_msg' closure with the argument 100
    print_msg(100);

    // Call the 'print_msg' closure with the argument 200
    print_msg(200);

    // Call the 'print_msg' closure with the argument 300
    print_msg(300);
}
