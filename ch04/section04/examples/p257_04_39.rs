// ▼ List 4-39
// Import the 'calc' module from the 'mymodule' crate
use mymodule::calc;

fn main() {
    // Define an integer variable
    let x = 123; //☆

    // Call the 'is_prime' function from the 'calc' module to check if the number is prime
    let res = calc::is_prime(x);

    // Print the result
    println!("answer: {} = {}", x, res);
}
