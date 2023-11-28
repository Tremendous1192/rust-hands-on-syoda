// ▼ List 4-35
// Import the entire 'mymodule' module
use mymodule;

fn main() {
    // Define two integer variables
    let x = 10;
    let y = 20;

    // Call the 'add' function from the 'mymodule' module to calculate the sum
    let res = mymodule::add(x, y);

    // Print the result
    println!("answer: {} + {} = {}", x, y, res);
}
