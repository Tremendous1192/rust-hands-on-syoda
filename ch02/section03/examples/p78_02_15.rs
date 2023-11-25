fn main() {
    let data = [12, 34, 56, 78, 90]; // Initialize an array named 'data' with specified values
    let mut ans = 0; // Initialize the variable to store the sum

    // Use a for loop to iterate over each item in the 'data' array
    for item in data {
        ans += item; // Add the current item to the sum
    }

    // Print the result, which is the sum of the elements in the 'data' array
    println!("The sum of data is {}.", ans);
}
