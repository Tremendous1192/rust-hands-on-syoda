fn main() {
    let num: u8 = 7; // Declare and initialize the variable 'num'

    // Match the value of 'num' against different cases and print corresponding messages
    match num {
        1 => println!("{} is the month of New Year's Day.", num),
        2 => println!("{} is the month of Setsubun.", num),
        3 => println!("{} is the month of Hinamatsuri.", num),
        4 => println!("{} is the month of school entrance ceremonies.", num),
        5 => println!("{} is the month of Golden Week.", num),
        6 => println!("{} is the month of the rainy season.", num),
        7 => println!("{} is the month when summer vacation begins.", num),
        8 => println!("{} is the month of the Obon holiday.", num),
        9 => println!("{} is the month of the new school year.", num),
        10 => println!("{} is the month of Halloween.", num),
        11 => println!("{} is the month of Black Friday.", num),
        12 => println!("{} is the month of Christmas.", num),
        _ => println!("There is no month corresponding to {}.", num),
    }
}
