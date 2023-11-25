fn main() {
    // Create a string slice 'msg' with the value "Hello!"
    let msg = "Hello!";

    // Copy the value of 'msg' to 'msg2' (string slices are Copy)
    let msg2 = msg;

    // Print the original string slice 'msg'
    println!("{}", msg);

    // Print the copied string slice 'msg2'
    println!("{}", msg2);
}
