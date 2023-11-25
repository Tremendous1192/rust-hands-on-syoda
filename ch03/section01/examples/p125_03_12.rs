fn main() {
    // Create a mutable string 'msg' with the content "Hello, world!"
    let mut msg = String::from("Hello, world!");

    // Create a new string 'world' by slicing 'msg' from index 7 to 12 (exclusive)
    let world = String::from(&msg[7..12]);

    // Print the sliced string 'world' and its original source string 'msg'
    println!("`{}` in `{}`.", world, msg);

    // Insert the string "RUST?" into 'msg' at index 7
    msg.insert_str(7, "RUST?");

    // Create a new string 'world' by slicing 'msg' from index 7 to 12 (exclusive)
    let mut world = String::from(&msg[7..12]);

    // Append the character '!' to the string 'world'
    world.push('!');

    // Print the modified sliced string 'world' and its updated source string 'msg'
    println!("`{}` in `{}`.", world, msg);
}
