fn main() {
    let max = 100; // Define the maximum value for the sum
    let mut ans = 0; // Initialize the variable to store the sum
    let mut count = 1; // Initialize the counter variable

    // Start an infinite loop
    loop {
        // Check if the counter exceeds the maximum value
        if count > max {
            break; // Exit the loop if the condition is met
        }

        ans += count; // Add the current value of the counter to the sum
        count += 1; // Increment the counter
    }

    // Print the result, which is the sum of numbers from 1 to the specified maximum
    println!("The sum of numbers from 1 to {} is {}.", max, ans);
}
