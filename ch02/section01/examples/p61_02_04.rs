fn main() {
    // Declare and initialize variables
    let x = 123;
    let y = 45;

    // Perform addition and print the result
    let z = x + y;
    println!("{} + {} = {}", x, y, z);

    // Perform subtraction and print the result (shadowing the variable z)
    let z = x - y;
    println!("{} - {} = {}", x, y, z);
}
