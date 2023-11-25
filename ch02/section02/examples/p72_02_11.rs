fn main() {
    let max = 100; // Specify the maximum value for summation
    let mut ans = 0; // Initialize the variable to store the sum
    let mut count = 1; // Initialize the counter variable

    // Use a while loop to iterate until the counter reaches the maximum value
    while count <= max {
        ans += count; // Add the current value of the counter to the sum
        count += 1; // Increment the counter
    }

    // Print the result, which is the sum of numbers from 1 to the specified maximum
    println!("The sum of numbers from 1 to {} is {}.", max, ans);
}
