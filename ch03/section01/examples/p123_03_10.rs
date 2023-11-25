fn main() {
    // Create a string slice 'msg' with the content "Hello, world!"
    let msg = "Hello, world!";

    // Create a string slice 'world' by slicing 'msg' from index 7 to 12 (exclusive)
    let world = &msg[7..12];

    // Print the sliced string 'world' and its original source string 'msg'
    println!("`{}` in `{}`.", world, msg);
}
