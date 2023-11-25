fn main() {
    let max = 100; // Specify the maximum value for summation
    let mut ans = 0; // Initialize the variable to store the sum

    // Use a for loop to iterate over the range from 1 to the specified maximum (inclusive)
    for item in 1..=max {
        ans += item; // Add the current value of the iteration to the sum
    }

    // Print the result, which is the sum of numbers from 1 to the specified maximum
    println!("The sum of numbers from 1 to {} is {}.", max, ans);
}
