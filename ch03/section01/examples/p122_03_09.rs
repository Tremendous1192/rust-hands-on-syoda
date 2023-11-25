fn main() {
    // Create a string slice 'msg' with the content "Hello!"
    let msg = "Hello!";

    // Create a reference 'msg_p' pointing to 'msg'
    let msg_p = &msg;

    // Dereference 'msg_p' to obtain the value 'msg_v'
    let msg_v = *msg_p;

    // Print the original string 'msg', its reference 'msg_p', and the dereferenced value 'msg_v'
    println!("{}, {}, {}.", msg, msg_p, msg_v);
}
