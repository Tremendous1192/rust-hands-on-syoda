fn main() {
    // Initialize a mutable variable 'x' with the value 10
    let mut x = 10;

    // Define a mutable closure 'double' that multiplies 'x' by 2 and returns the result
    let mut double = || {
        x *= 2;
        x
    };

    // Call the 'double' closure and print the result
    println!("x = {}.", double());

    // Call the 'double' closure again with the updated value of 'x' and print the result
    println!("x = {}.", double());

    // Call the 'double' closure once more with the further updated value of 'x' and print the result
    println!("x = {}.", double());
}
