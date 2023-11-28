// ▼ List 4-37
// Import the 'add' function from the 'calc' module within the 'mymodule' crate
use mymodule::calc;

fn main() {
    // Define two integer variables
    let x = 10;
    let y = 20;

    // Call the 'add' function from the 'calc' module to calculate the sum
    let res = calc::add(x, y);

    // Print the result
    println!("answer: {} + {} = {}", x, y, res);
}
