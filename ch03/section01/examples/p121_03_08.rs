fn main() {
    // Create a mutable String 'msg'
    let mut msg = String::from("Hello!");

    // Print the initial value of 'msg'
    println!("msg: {}", msg);

    // Call the function 'print_msg' with a mutable reference to 'msg'
    print_msg(&mut msg);

    // Print the modified value of 'msg' after calling the function
    println!("msg: {}", msg);
}

// Function to modify a message using a mutable reference to a String
fn print_msg(msg: &mut String) {
    // Append additional content to the existing message
    msg.push_str("!!!!");

    // Print the modified message
    println!("Message is \"{}\".", msg);
}
