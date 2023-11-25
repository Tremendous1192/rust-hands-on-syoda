fn main() {
    // Create a reference to a String 'msg'
    let msg = &String::from("Hello!");

    // Print the initial value of 'msg'
    println!("msg: {}", msg);

    // Create a new scope with a shadowed variable 'msg'
    {
        // Call the function 'print_msg' with a reference to 'msg'
        // The returned value is assigned to the shadowed variable 'msg'
        let msg = print_msg(msg);

        // Print the modified value of 'msg' within the inner scope
        println!("msg: {}", msg);
    }

    // Print the original value of 'msg' outside the inner scope
    println!("msg: {}", msg);
}

// Function to modify and print a message using a reference to a String
fn print_msg(msg: &String) -> String {
    // Create a new String with additional content and assign it to 'msg'
    let msg = String::from("*** ") + msg + " ***";

    // Print the modified message
    println!("Message is \"{}\".", msg);

    // Return the modified message
    msg
}
