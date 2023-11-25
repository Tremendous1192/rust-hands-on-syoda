fn main() {
    let num = 123; // Declare and initialize the variable 'num'

    // Check divisibility by different numbers and print the result
    if num % 5 == 0 {
        println!("{} is divisible by 5.", num);
    } else if num % 4 == 0 {
        println!("{} is divisible by 4.", num);
    } else if num % 3 == 0 {
        println!("{} is divisible by 3.", num);
    } else if num % 2 == 0 {
        println!("{} is divisible by 2.", num);
    } else {
        println!("{} is not divisible by 2, 3, 4, or 5.", num);
    }
}
