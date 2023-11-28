// ▼ List 4-34
// Import the 'add' function from the 'mymodule' module
use mymodule::add;

fn main() {
    // Define two integer variables
    let x = 10;
    let y = 20;

    // Call the 'add' function to calculate the sum
    let res = add(x, y);

    // Print the result
    println!("answer: {} + {} = {}", x, y, res);
}
