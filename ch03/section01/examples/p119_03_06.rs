fn main() {
    // Create a String variable 'msg' with the value "Hello!"
    let msg = String::from("Hello!");

    // Call the function 'print_msg' with a reference to 'msg'
    print_msg(&msg);

    // 'msg' can still be used after passing a reference to it
    println!("msg: {}", msg);
}

// Function to print a message using a reference to a String
fn print_msg(msg: &String) {
    // Print the message passed as a reference
    println!("Message is \"{}\".", msg);
}
