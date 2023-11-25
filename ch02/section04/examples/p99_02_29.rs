fn main() {
    // Call the hello function with the name "taro"
    hello(String::from("taro"));

    // Call the hello function with the name "hanako"
    hello(String::from("hanako"));
}

fn hello(name: String) {
    // Print a greeting using the provided name
    println!("Hello, {}!", name);
}
