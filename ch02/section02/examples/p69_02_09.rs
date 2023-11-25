fn main() {
    let num: u8 = 7; // Declare and initialize the variable 'num'

    // Match the value of 'num' against different cases and print corresponding messages
    match num {
        1 => println!("{} is the month of New Year's Day.", num),
        2 => println!("{} is the month of winter.", num),
        3 | 4 | 5 => println!("{} is the month of spring.", num),
        6 | 7 | 8 => println!("{} is the month of summer.", num),
        9 | 10 | 11 => println!("{} is the month of autumn.", num),
        12 => println!("{} is the month of December, the year-end rush.", num),
        _ => println!("There is no month corresponding to {}.", num),
    }
}
