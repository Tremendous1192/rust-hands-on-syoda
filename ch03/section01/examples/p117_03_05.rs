fn main() {
    // Create a mutable String variable 'msg' with the value "Hello!"
    let mut msg = String::from("Hello!");

    // Call the function 'print_msg' with the 'msg' as an argument, and update 'msg' with the result
    msg = print_msg(msg);

    // Print the updated value of 'msg'
    println!("msg: {}", msg);
}

// Function to print a message and return the same message
fn print_msg(msg: String) -> String {
    // Print the message passed as an argument
    println!("Message is \"{}\".", msg);

    // Return the same message
    msg
}
